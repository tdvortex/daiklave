use eyre::{eyre, Result, WrapErr};
use sqlx::{query, PgPool, Postgres, Transaction};

use crate::{character::Character, abilities::update::update_abilities, craft::update::update_craft, attributes::update::update_attributes, health::update::update_health, intimacies::update::update_intimacies, weapons::update::update_weapons, armor::update::update_armor, merits::update::update_merits, martial_arts::update::update_martial_arts};

use super::{create::create_character_transaction, retrieve::retrieve_character_transaction};

#[derive(Debug, Default)]
pub struct CharacterBaseDiff(Option<(String, Option<String>, i16, i16, i16, i16)>);

impl Character {
    pub fn compare_newer_base(&self, newer: &Character) -> CharacterBaseDiff {
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


pub async fn update_base_character(
    base_character_diff: CharacterBaseDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    if base_character_diff.0.is_none() {
        return Ok(());
    }

    let (
        name,
        maybe_concept,
        current_willpower,
        maximum_willpower,
        current_experience,
        total_experience,
    ) = base_character_diff.0.as_ref().unwrap();

    query!("
        UPDATE characters
        SET name = $2, concept = $3, current_willpower = $4, max_willpower = $5, current_experience = $6, total_experience = $7
        WHERE id = $1",
        character_id, name.as_ref() as &str, maybe_concept.as_deref(), current_willpower, maximum_willpower, current_experience, total_experience
    ).execute(&mut *transaction).await.wrap_err_with(|| format!("Failed to update character: {:?}", base_character_diff.0.as_ref().unwrap()))?;

    Ok(())
}


pub async fn update_character(pool: &PgPool, character: &Character) -> Result<Character> {
    let mut transaction = pool.begin().await.wrap_err("Failed to start transaction")?;

    let old_character = if character.id.is_placeholder() {
        create_character_transaction(&mut transaction, *character.player.id())
            .await
            .wrap_err_with(|| {
                format!("Failed to create initial character from: {:#?}", character)
            })?
    } else {
        retrieve_character_transaction(&mut transaction, *character.id)
            .await
            .wrap_err_with(|| {
                format!(
                    "Database error on retrieving pre-update character_id: {}",
                    *character.id
                )
            })?
            .ok_or_else(|| eyre!("No character found with id {}", *character.id))?
    };

    let character_id = if old_character.id.is_placeholder() {
        return Err(eyre!(
            "Missing character id for character with name {}",
            old_character.name
        ));
    } else {
        *old_character.id
    };

    let diff = old_character.compare_newer(&character);

    update_abilities(diff.abilities_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating abilities")?;
    update_craft(diff.craft_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating craft abilities")?;
    update_attributes(diff.attributes_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating attributes")?;
    update_base_character(diff.base_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating base character")?;
    update_health(diff.health_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating health")?;
    update_intimacies(diff.intimacies_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating intimacies")?;
    update_weapons(diff.weapons_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating weapons")?;
    update_armor(diff.armor_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating armor")?;
    update_merits(diff.merits_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating merits")?;
    update_martial_arts(diff.martial_arts_diff, &mut transaction, character_id)
        .await
        .wrap_err("Error when updating martial arts")?;

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