#[path = "../data.rs"]
mod data;

// use std::time::Duration;
use sqlx::postgres::PgPool;
use sqlx::Error;

use crate::data::Message;

pub async fn consume_json(pool: &PgPool, message: Message) -> Result<i64, Error> {
	let mut ingredients = Vec::new();
	for ingredient in message.ingredients {
		ingredients.extend_from_slice(ingredient.replace("#", "-").as_bytes());
		ingredients.push("#".as_bytes()[0])
	}

	let rec = sqlx::query!(
		r#"
			INSERT INTO "Recipes" (name, description, ingredients)
			VALUES ( $1, $2, $3 )
			RETURNING id
		"#,
		message.name,
		message.description,
		ingredients
	)
	.fetch_one(pool)
	.await?;

	Ok(rec.id)
}
