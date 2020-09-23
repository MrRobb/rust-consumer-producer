use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub enum Operation {
	Insert,
	Update,
	Delete,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
	pub op: Operation,
	pub name: String,
	pub description: String,
	pub ingredients: Vec<String>,
}
