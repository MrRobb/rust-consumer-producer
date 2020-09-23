#[path = "../data.rs"]
mod data;

use amiquip::{Connection, Exchange, Publish, Result};
use clap::App;
use clap::Arg;
use data::Message;
use data::Operation;

fn emit(url: &str, channel: Option<u16>, routing_name: &str, n_messages: usize) -> Result<()> {
	// Open connection
	let mut connection = Connection::insecure_open(url)?;

	// Open channel
	let channel = connection.open_channel(channel)?;

	// Get exchange
	let exchange = Exchange::direct(&channel);

	// Publish a message
	for i in 0..n_messages {
		let message = Message {
			op: Operation::Insert,
			name: format!("Random recipe name {}", i),
			description: format!("Simple description"),
			ingredients: (0..10).map(|n| n.to_string()).collect(),
		};

		exchange.publish(Publish::new(
			serde_json::to_string(&message).unwrap().as_bytes(),
			routing_name,
		))?;
	}

	connection.close()
}

fn main() -> Result<()> {
	let config = App::new("Producer program")
		.about("Sends messages to a consumer queue")
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
			Arg::new("n-messages")
				.short('n')
				.long("n-messages")
				.about("Number messages to send")
				.value_name("N_MESSAGES")
				.default_value("1"),
		)
		.get_matches();

	emit(
		config.value_of("url").unwrap(),
		Some(config.value_of_t("channel").unwrap()),
		config.value_of("queue").unwrap(),
		config.value_of_t("n-messages").unwrap(),
	)
}
