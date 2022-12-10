use eyre::{eyre, Result, WrapErr};
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

        let eq_condition = (self.name.as_str() == newer.name.as_str())
            && (self.concept.as_deref() == newer.concept.as_deref())
            && (self.willpower.current == newer.willpower.current)
            && (self.willpower.maximum == newer.willpower.maximum)
            && (self.experience.current.min(i16::MAX as u16)
                != newer.experience.current.max(i16::MAX as u16))
            && (self.experience.total.min(i16::MAX as u16)
                != newer.experience.total.max(i16::MAX as u16));

        if !eq_condition {
            diff = CharacterBaseDiff(Some((
                newer.name.clone(),
                newer.concept.clone(),
                newer.willpower.current as i16,
                newer.willpower.maximum as i16,
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
            )));
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
            character_id, name.as_ref() as &str, maybe_concept.as_deref(), current_willpower, maximum_willpower, current_experience, total_experience
        ).execute(&mut *transaction).await.wrap_err_with(|| format!("Failed to update character: {:?}", self.0.as_ref().unwrap()))?;

        Ok(())
    }
}

pub async fn update_character(pool: &PgPool, character: &Character) -> Result<Character> {
    let mut transaction = pool.begin().await.wrap_err("Failed to start transaction")?;

    let old_character = if character.id.is_none() {
        create_character_transaction(&mut transaction, character.player.clone())
            .await
            .wrap_err_with(|| {
                format!("Failed to create initial character from: {:#?}", character)
            })?
    } else {
        retrieve_character_transaction(&mut transaction, character.id.unwrap())
            .await
            .wrap_err_with(|| {
                format!(
                    "Database error on retrieving pre-update character_id: {}",
                    character.id.unwrap()
                )
            })?
            .ok_or_else(|| eyre!("No character found with id {}", character.id.unwrap()))?
    };

    let character_id = old_character.id.ok_or_else(|| {
        eyre!(
            "Missing character id for character with name {}",
            old_character.name
        )
    })?;

    old_character
        .abilities
        .compare_newer(&character.abilities)
        .update(&mut transaction, character_id)
        .await
        .wrap_err("Error when updating abilities")?;
    old_character
        .attributes
        .compare_newer(&character.attributes)
        .update(&mut transaction, character_id)
        .await
        .wrap_err("Error when updating attributes")?;
    old_character
        .compare_newer(character)
        .update(&mut transaction, character_id)
        .await
        .wrap_err("Error when updating base character")?;
    old_character
        .health
        .compare_newer(&character.health)
        .update(&mut transaction, character_id)
        .await
        .wrap_err("Error when updating health")?;
    compare_intimacies(&old_character.intimacies, &character.intimacies)
        .update(&mut transaction, character_id)
        .await
        .wrap_err("Error when updating intimacies")?;
    old_character
        .weapons
        .compare_newer(&character.weapons)
        .update(&mut transaction, character_id)
        .await
        .wrap_err("Error when updating weapons")?;
    old_character
        .armor
        .compare_newer(&character.armor)
        .update(&mut transaction, character_id)
        .await
        .wrap_err("Error when updating armor")?;
    compare_merits(&old_character.merits, &character.merits)
        .update(&mut transaction, character_id)
        .await
        .wrap_err("Error when updating merits")?;

    let character = retrieve_character_transaction(&mut transaction, character_id)
        .await
        .wrap_err_with(|| {
            format!(
                "Database error on retrieving post-update character_id: {}",
                character_id
            )
        })?
        .ok_or_else(|| {
            eyre!(
                "Could not retrieve post-update character with id {}",
                character_id
            )
        })?;

    transaction
        .commit()
        .await
        .wrap_err("Error trying to commit character update transaction")?;

    Ok(character)
}
