use std::collections::HashSet;

use crate::abilities::tables::AbilityNamePostgres;
use crate::abilities::Abilities;
use eyre::{Context, Result};
use sqlx::{query, Postgres, Transaction};

use super::{AbilityNameVanilla};

#[derive(Debug, Default)]
pub struct AbilitiesDiff {
    abilities_to_modify: Vec<(AbilityNameVanilla, u8)>,
    specialties_to_add: Vec<(AbilityNameVanilla, String)>,
    specialties_to_remove: Vec<(AbilityNameVanilla, String)>,
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

                    for specialty in new_specialties.into_iter() {
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

impl AbilitiesDiff {
    async fn update_vanilla_abilities(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        let (mut names_to_update, mut dots_to_update) = (Vec::<AbilityNamePostgres>::new(), Vec::new());

        for (name_vanilla, dots) in self.abilities_to_modify.iter() {
            names_to_update.push(
                (*name_vanilla).into(),
            );
            dots_to_update.push((*dots).into());
        }

        if !names_to_update.is_empty() {
            query!(
                "UPDATE abilities
                SET dots = data.dots
                FROM UNNEST($2::ABILITYNAME[], $3::SMALLINT[]) as data(name, dots)
                WHERE abilities.character_id = $1 AND abilities.name = data.name",
                character_id,
                &names_to_update as &[AbilityNamePostgres],
                &dots_to_update as &[i16]
            )
            .execute(&mut *transaction)
            .await
            .wrap_err("Database error attempting to update non-Craft, non-MartialArts abilities")?;
        }

        Ok(())
    }

    async fn remove_specialties(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        let ability_name_with_specialty_to_remove: Vec<AbilityNamePostgres> = self
            .specialties_to_remove
            .iter()
            .map(|(ability_name, _)| (*ability_name).into())
            .collect();

        let specialty_name_to_remove: Vec<&str> = self
            .specialties_to_remove
            .iter()
            .map(|(_, specialty)| specialty.as_str())
            .collect();

        query!("
            DELETE FROM specialties
            WHERE (specialties.ability_id, specialties.specialty) IN
            (
                SELECT
                    abilities.id,
                    data.specialty
                FROM
                    abilities 
                    INNER JOIN UNNEST($2::ABILITYNAME[], $3::VARCHAR(255)[]) AS data(ability_name, specialty)
                    ON (abilities.name = data.ability_name)
                    INNER JOIN specialties ON (abilities.id = specialties.ability_id AND specialties.specialty = data.specialty)
                WHERE abilities.character_id = $1
            )",
        character_id,
        &ability_name_with_specialty_to_remove as &[AbilityNamePostgres],
        &specialty_name_to_remove as &[&str]
        ).execute(&mut *transaction).await.wrap_err("Database error attempting to remove specialties")?;

        Ok(())
    }

    async fn add_specialties(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        let ability_name_with_specialty_to_add: Vec<AbilityNamePostgres> = self
            .specialties_to_add
            .iter()
            .map(|(ability_name, _)| (*ability_name).into())
            .collect();

        let specialty_name_to_add: Vec<&str> = self
            .specialties_to_add
            .iter()
            .map(|(_, text)| text.as_str())
            .collect();

        query!(
            r#"
            INSERT INTO specialties
            SELECT
                abilities.id,
                added.specialty
            FROM abilities INNER JOIN (
                SELECT 
                    name, 
                    specialty
                FROM UNNEST($2::ABILITYNAME[], $3::VARCHAR(255)[]) AS data(name, specialty)
            ) AS added 
            ON (abilities.name = added.name)
            WHERE abilities.character_id = $1::INTEGER
            "#,
            character_id as i32,
            &ability_name_with_specialty_to_add as &[AbilityNamePostgres],
            &specialty_name_to_add as &[&str],
        )
        .execute(&mut *transaction)
        .await
        .wrap_err("Database error attempting to insert specialties")?;

        Ok(())
    }

    pub async fn update(
        self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        if !self.abilities_to_modify.is_empty() {
            self.update_vanilla_abilities(transaction, character_id)
                .await
                .wrap_err("Error attempting to upsert abilities")?;
        }

        if !self.specialties_to_remove.is_empty() {
            self.remove_specialties(transaction, character_id)
                .await
                .wrap_err("Error attempting to remove specialties")?;
        }

        if !self.specialties_to_add.is_empty() {
            self.add_specialties(transaction, character_id)
                .await
                .wrap_err("Error attempting to add specialties")?;
        }

        Ok(())
    }
}
