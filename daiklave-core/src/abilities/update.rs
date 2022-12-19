use std::collections::HashSet;

use crate::abilities::tables::AbilityNameVanillaPostgres;
use crate::abilities::Abilities;
use eyre::{Context, Result};
use sqlx::{query, Postgres, Transaction};

use super::AbilityNameVanilla;

#[derive(Debug, Default)]
pub struct AbilitiesDiff {
    pub abilities_to_modify: Vec<(AbilityNameVanilla, u8)>,
    pub specialties_to_add: Vec<(AbilityNameVanilla, String)>,
    pub specialties_to_remove: Vec<(AbilityNameVanilla, String)>,
}

impl Abilities {
    pub fn compare_newer(&self, newer: &Self) -> AbilitiesDiff {
        let mut diff = AbilitiesDiff::default();

        for (old_ability, new_ability) in self.iter().zip(newer.iter()) {
            if old_ability.dots() != new_ability.dots() {
                diff.abilities_to_modify.push((
                    old_ability.name().without_subskill().try_into().unwrap(),
                    new_ability.dots(),
                ));
            }

            match (old_ability.specialties(), new_ability.specialties()) {
                (None, None) => {}
                (None, Some(added)) => {
                    for specialty in added.clone().into_iter() {
                        diff.specialties_to_add.push((
                            new_ability.name().without_subskill().try_into().unwrap(),
                            specialty,
                        ));
                    }
                }
                (Some(removed), None) => {
                    for specialty in removed.clone().into_iter() {
                        diff.specialties_to_remove.push((
                            old_ability.name().without_subskill().try_into().unwrap(),
                            specialty,
                        ));
                    }
                }
                (Some(old_specialties), Some(new_specialties)) => {
                    let mut old_set: HashSet<&str> =
                        old_specialties.iter().map(|s| s.as_str()).collect();

                    for specialty in new_specialties.iter() {
                        if !old_set.remove(specialty.as_str()) {
                            diff.specialties_to_add.push((
                                new_ability.name().without_subskill().try_into().unwrap(),
                                specialty.clone(),
                            ));
                        }
                    }

                    for specialty in old_set.into_iter() {
                        diff.specialties_to_remove.push((
                            old_ability.name().without_subskill().try_into().unwrap(),
                            specialty.to_owned(),
                        ));
                    }
                }
            }
        }
        diff
    }
}

async fn update_vanilla_abilities(
    abilities_diff: &AbilitiesDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    let (mut names_to_update, mut dots_to_update) =
        (Vec::<AbilityNameVanillaPostgres>::new(), Vec::new());

    for (name_vanilla, dots) in abilities_diff.abilities_to_modify.iter() {
        names_to_update.push((*name_vanilla).into());
        dots_to_update.push((*dots).into());
    }

    if !names_to_update.is_empty() {
        query!(
            "UPDATE abilities
            SET dots = data.dots
            FROM UNNEST($2::ABILITYNAMEVANILLA[], $3::SMALLINT[]) as data(name, dots)
            WHERE abilities.character_id = $1 AND abilities.name = data.name",
            character_id,
            &names_to_update as &[AbilityNameVanillaPostgres],
            &dots_to_update as &[i16]
        )
        .execute(&mut *transaction)
        .await
        .wrap_err("Database error attempting to update non-Craft, non-MartialArts abilities")?;
    }

    Ok(())
}

async fn remove_specialties(
    abilities_diff: &AbilitiesDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    let ability_name_with_specialty_to_remove: Vec<AbilityNameVanillaPostgres> = abilities_diff
        .specialties_to_remove
        .iter()
        .map(|(ability_name, _)| (*ability_name).into())
        .collect();

    let specialty_name_to_remove: Vec<&str> = abilities_diff
        .specialties_to_remove
        .iter()
        .map(|(_, specialty)| specialty.as_str())
        .collect();

    query!(
        "
        DELETE FROM specialties
        WHERE (specialties.character_id, specialties.ability_name, specialties.specialty) IN
        (
            SELECT
                $1::INTEGER as character_id,
                data.ability_name as ability_name,
                data.specialty as specialty
            FROM UNNEST($2::ABILITYNAMEVANILLA[], $3::VARCHAR(255)[]) AS data(ability_name, specialty)
        )",
        character_id as i32,
        &ability_name_with_specialty_to_remove as &[AbilityNameVanillaPostgres],
        &specialty_name_to_remove as &[&str]
    )
    .execute(&mut *transaction)
    .await
    .wrap_err("Database error attempting to remove specialties")?;

    Ok(())
}

async fn add_specialties(
    abilities_diff: &AbilitiesDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    let ability_name_with_specialty_to_add: Vec<AbilityNameVanillaPostgres> = abilities_diff
        .specialties_to_add
        .iter()
        .map(|(ability_name, _)| (*ability_name).into())
        .collect();

    let specialty_name_to_add: Vec<&str> = abilities_diff
        .specialties_to_add
        .iter()
        .map(|(_, text)| text.as_str())
        .collect();

    query!(
        r#"
        INSERT INTO specialties
        SELECT
            $1::INTEGER as character_id,
            data.name as name,
            data.specialty as specialty
        FROM UNNEST($2::ABILITYNAMEVANILLA[], $3::VARCHAR(255)[]) AS data(name, specialty)
        "#,
        character_id as i32,
        &ability_name_with_specialty_to_add as &[AbilityNameVanillaPostgres],
        &specialty_name_to_add as &[&str],
    )
    .execute(&mut *transaction)
    .await
    .wrap_err("Database error attempting to insert specialties")?;

    Ok(())
}

pub async fn update_abilities(
    abilities_diff: AbilitiesDiff,
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
) -> Result<()> {
    if !abilities_diff.abilities_to_modify.is_empty() {
        update_vanilla_abilities(&abilities_diff, transaction, character_id)
            .await
            .wrap_err("Error attempting to upsert abilities")?;
    }

    if !abilities_diff.specialties_to_remove.is_empty() {
        remove_specialties(&abilities_diff,transaction, character_id)
            .await
            .wrap_err("Error attempting to remove specialties")?;
    }

    if !abilities_diff.specialties_to_add.is_empty() {
        add_specialties(&abilities_diff,transaction, character_id)
            .await
            .wrap_err("Error attempting to add specialties")?;
    }

    Ok(())
}