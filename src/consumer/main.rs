// Port of https://www.rabbitmq.com/tutorials/tutorial-one-python.html. Run this
// in one shell, and run the hello_world_publish example in another.
#[path = "../data.rs"]
mod data;
#[path = "process.rs"]
mod process;

use amiquip::Connection;
use amiquip::ConsumerMessage;
use amiquip::ConsumerOptions;
use amiquip::QueueDeclareOptions;
use amiquip::Result;
use clap::App;
use clap::Arg;
use data::Message;
use serde_json;
use sqlx::PgPool;

async fn listen<'a>(
	url: &str,
	channel: Option<u16>,
	queue_name: &str,
	queue_options: QueueDeclareOptions,
	consumer_options: ConsumerOptions,
	db_pool: PgPool,
) -> Result<()> {
	// Open connection
	// TODO: Add TLS support for a secure connection
	let mut connection = Connection::insecure_open(url)?;

	// Open a channel
	let channel = connection.open_channel(channel)?;

	// Declare queue
	let queue = channel.queue_declare(queue_name, queue_options)?;

	// Start consumer process
	let consumer = queue.consume(consumer_options)?;

	for (_, message) in consumer.receiver().iter().enumerate() {
		match message {
			ConsumerMessage::Delivery(delivery) => match serde_json::from_slice::<Message>(delivery.body.as_slice()) {
				Ok(json) => {
					if process::consume_json(&db_pool, json).await.is_ok() {
						consumer.ack(delivery)?;
					}
				},
				Err(_) => todo!(),
			},
			_ => break,
		}
	}

	println!("Connection closed");
	connection.close()
}

async fn connect(url: &str) -> PgPool {
	PgPool::connect(url).await.unwrap()
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let config = App::new("Consumer program")
		.about("Processes messages from a consumer queue")
		.arg(
			Arg::new("url")
				.long("url")
				.about("URL of the service running RabbitMQ")
				.value_name("RABBITMQ_URL")
				.default_value("amqp://guest:guest@localhost:5672"),
		)
		.arg(
			Arg::new("channel")
				.long("channel")
				.about("Number of the channel selected")
				.value_name("CHANNEL_ID")
				.default_value("1"),
		)
		.arg(
			Arg::new("queue")
				.long("queue")
				.about("Name of the queue")
				.value_name("QUEUE_NAME")
				.default_value("sample"),
		)
		.arg(
			Arg::new("database-url")
				.long("db")
				.about("URL of the database service")
				.value_name("DATABASE_URL")
				.default_value("postgres://guest:guest@localhost:5432/guestdb"),
		)
		.get_matches();

	listen(
		config.value_of("url").unwrap(),
		Some(config.value_of_t("channel").unwrap()),
		config.value_of("queue").unwrap(),
		QueueDeclareOptions::default(),
		ConsumerOptions::default(),
		connect(config.value_of("database-url").unwrap()).await,
	)
	.await?;

	Ok(())
}
