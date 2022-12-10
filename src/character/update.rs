use eyre::{eyre, Result};
use sqlx::{query, PgPool, Postgres, Transaction};

use crate::{
    character::Character, intimacies::update::compare_intimacies, merits::update::compare_merits,
};

use super::{create::create_character_transaction, retrieve::retrieve_character_transaction};

#[derive(Debug, Default)]
pub struct CharacterBaseDiff(Option<(String, Option<String>, i16, i16, i16, i16)>);

impl Character {
    pub fn compare_newer(&self, newer: &Character) -> CharacterBaseDiff {
        let mut diff = CharacterBaseDiff::default();

        if self.name != newer.name {
            diff = CharacterBaseDiff(Some((
                newer.name.to_owned(),
                newer.concept.as_ref().map(String::to_owned),
                newer.willpower.current.into(),
                newer.willpower.maximum.into(),
                // No one should ever have more than 32,767 experience
                newer
                    .experience
                    .current
                    .min(i16::MAX as u16)
                    .try_into()
                    .unwrap(),
                newer
                    .experience
                    .total
                    .min(i16::MAX as u16)
                    .try_into()
                    .unwrap(),
            )))
        }

        diff
    }
}

impl CharacterBaseDiff {
    pub async fn update(
        self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        if self.0.is_none() {
            return Ok(());
        }

        let (
            name,
            maybe_concept,
            current_willpower,
            maximum_willpower,
            current_experience,
            total_experience,
        ) = self.0.as_ref().unwrap();

        query!("
            UPDATE characters
            SET name = $2, concept = $3, current_willpower = $4, max_willpower = $5, current_experience = $6, total_experience = $7
            WHERE id = $1",
            character_id, name, maybe_concept.as_deref(), current_willpower, maximum_willpower, current_experience, total_experience
        ).execute(&mut *transaction).await?;

        Ok(())
    }
}

pub async fn update_character(pool: &PgPool, character: Character) -> Result<Character> {
    let mut transaction = pool.begin().await?;

    let old_character = if character.id.is_none() {
        create_character_transaction(&mut transaction, character.player.clone()).await?
    } else {
        retrieve_character_transaction(&mut transaction, character.id.unwrap())
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
        .update(&mut transaction, character_id)
        .await?;
    old_character
        .attributes
        .compare_newer(&character.attributes)
        .update(&mut transaction, character_id)
        .await?;
    old_character
        .compare_newer(&character)
        .update(&mut transaction, character_id)
        .await?;
    old_character
        .health
        .compare_newer(&character.health)
        .update(&mut transaction, character_id)
        .await?;
    compare_intimacies(&old_character.intimacies, &character.intimacies)
        .update(&mut transaction, character_id)
        .await?;
    old_character
        .weapons
        .compare_newer(&character.weapons)
        .update(&mut transaction, character_id)
        .await?;
    old_character
        .armor
        .compare_newer(&character.armor)
        .update(&mut transaction, character_id)
        .await?;
    compare_merits(&old_character.merits, &character.merits)
        .update(&mut transaction, character_id)
        .await?;

    let character = retrieve_character_transaction(&mut transaction, character_id)
        .await?
        .ok_or_else(|| eyre!("could not retrieve put character with id {}", character_id))?;

    transaction.commit().await?;

    Ok(character)
}
