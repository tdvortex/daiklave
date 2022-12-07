use crate::{character::Character, database::character_diff::compare_intimacies};
use eyre::{eyre, Result};
use sqlx::PgPool;

use super::{get_character::get_character_transaction, post_character::post_character_transaction};

pub async fn put_character(pool: &PgPool, character: Character) -> Result<Character> {
    let mut transaction = pool.begin().await?;

    let old_character = if character.id.is_none() {
        post_character_transaction(&mut transaction, character.player.clone()).await?
    } else {
        get_character_transaction(&mut transaction, character.id.unwrap())
            .await?
            .ok_or_else(|| eyre!("no character found with id {}", character.id.unwrap()))?
    };

    let character_id = old_character.id.ok_or_else(|| {
        eyre!(
            "missing character id for character with name {}",
            old_character.name
        )
    })?;

    old_character
        .abilities
        .compare_newer(&character.abilities)
        .save(&mut transaction, character_id)
        .await?;
    old_character
        .attributes
        .compare_newer(&character.attributes)
        .save(&mut transaction, character_id)
        .await?;
    old_character
        .compare_newer(&character)
        .save(&mut transaction, character_id)
        .await?;
    old_character
        .health
        .compare_newer(&character.health)
        .save(&mut transaction, character_id)
        .await?;
    compare_intimacies(&old_character.intimacies, &character.intimacies)
        .save(&mut transaction, character_id)
        .await?;

    let character = get_character_transaction(&mut transaction, character_id)
        .await?
        .ok_or_else(|| eyre!("could not retrieve put character with id {}", character_id))?;

    transaction.commit().await?;

    Ok(character)
}
