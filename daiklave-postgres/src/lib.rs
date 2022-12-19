use daiklave_core::Character;
use sqlx::{PgPool, query};
use eyre::{WrapErr, Result};

pub async fn destroy_character(pool: &PgPool, id: i32) -> Result<()> {
    query!(
        "DELETE FROM characters
        WHERE id = $1",
        id as i32
    )
    .execute(pool)
    .await
    .wrap_err_with(|| format!("Database error deleting character {}", id))?;

    Ok(())
}


pub async fn retrieve_character(pool: &PgPool, character_id: i32) -> Result<Option<Character>> {
    todo!()
}


pub async fn update_character(pool: &PgPool, character: &Character) -> Result<Character> {
    todo!()
}

