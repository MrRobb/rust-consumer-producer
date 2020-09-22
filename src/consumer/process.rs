#[path = "../data.rs"]
mod data;

// use std::time::Duration;

use crate::data::Message;
use amiquip::Result;

pub fn consume_json(json: Message) -> Result<()> {
	println!("Message received {}", json.message_number);
	// std::thread::sleep(Duration::new(0, 1_000_000));
	Ok(())
}
