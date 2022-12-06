use sqlx::PgPool;
use eyre::Result;
use crate::character::{Character, self};

use super::get_character;

pub async fn put_character(pool: &PgPool, character: Character) -> Result<Character> {
    if character.id.is_none() {
        
    }


    let mut transaction = pool.begin().await?;




    let character = get_character(pool, character_id).await?;

    transaction.commit().await?;

    Ok(character)
}