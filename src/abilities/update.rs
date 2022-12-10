use crate::abilities::tables::AbilityNamePostgres;
use crate::abilities::Abilities;
use eyre::{Context, Result};
use sqlx::{query, Postgres, Transaction};

#[derive(Debug, Default)]
pub struct AbilitiesDiff {
    abilities_to_upsert: Vec<(AbilityNamePostgres, Option<String>, u8)>,
    abilities_to_remove: Vec<(AbilityNamePostgres, Option<String>)>,
    specialties_to_add: Vec<(AbilityNamePostgres, Option<String>, String)>,
    specialties_to_remove: Vec<(AbilityNamePostgres, Option<String>, String)>,
}

impl Abilities {
    pub fn compare_newer(&self, newer: &Self) -> AbilitiesDiff {
        let mut diff = AbilitiesDiff::default();

        for old_ability in self.iter() {
            match newer.get(
                old_ability.name().without_subskill(),
                old_ability.name().subskill(),
            ) {
                Some(new_ability) => {
                    if old_ability.dots() != new_ability.dots() {
                        diff.abilities_to_upsert.push((
                            old_ability.name().without_subskill().into(),
                            old_ability.name().subskill().map(|s| s.to_owned()),
                            new_ability.dots(),
                        ));
                    }

                    match (old_ability.specialties(), new_ability.specialties()) {
                        (None, None) => {}
                        (None, Some(specialties)) => {
                            diff.specialties_to_add
                                .extend(specialties.iter().map(|string_ref| {
                                    (
                                        old_ability.name().without_subskill().into(),
                                        old_ability.name().subskill().map(|s| s.to_owned()),
                                        string_ref.clone(),
                                    )
                                }));
                        }
                        (Some(specialties), None) => {
                            diff.specialties_to_remove.extend((*specialties).iter().map(
                                |string_ref| {
                                    (
                                        old_ability.name().without_subskill().into(),
                                        old_ability.name().subskill().map(|s| s.to_owned()),
                                        string_ref.clone(),
                                    )
                                },
                            ));
                        }
                        (Some(old), Some(new)) => {
                            diff.specialties_to_remove.extend(old.difference(new).map(
                                |specialty| {
                                    (
                                        old_ability.name().without_subskill().into(),
                                        old_ability.name().subskill().map(|s| s.to_owned()),
                                        specialty.clone(),
                                    )
                                },
                            ));

                            diff.specialties_to_add
                                .extend(new.difference(old).map(|specialty| {
                                    (
                                        old_ability.name().without_subskill().into(),
                                        old_ability.name().subskill().map(|s| s.to_owned()),
                                        specialty.clone(),
                                    )
                                }));
                        }
                    }
                }
                None => {
                    diff.abilities_to_remove.push((
                        old_ability.name().without_subskill().into(),
                        old_ability.name().subskill().map(|s| s.to_owned()),
                    ));
                }
            }
        }

        for new_ability in newer.iter() {
            if self
                .get(
                    new_ability.name().without_subskill(),
                    new_ability.name().subskill(),
                )
                .is_none()
            {
                diff.abilities_to_upsert.push((
                    new_ability.name().without_subskill().into(),
                    new_ability.name().subskill().map(|s| s.to_owned()),
                    new_ability.dots(),
                ));
                if new_ability.specialties().is_some() {
                    diff.specialties_to_add
                        .extend(new_ability.specialties().unwrap().iter().map(|string_ref| {
                            (
                                new_ability.name().without_subskill().into(),
                                new_ability.name().subskill().map(|s| s.to_owned()),
                                string_ref.clone(),
                            )
                        }));
                }
            }
        }

        diff
    }
}

impl AbilitiesDiff {
    async fn remove_abilities(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        let names_to_remove: Vec<AbilityNamePostgres> = self
            .abilities_to_remove
            .iter()
            .map(|(name, _subskill)| *name)
            .collect();

        let subskills_to_remove: Vec<Option<String>> = self
            .abilities_to_remove
            .iter()
            .map(|(_name, subskill)| subskill.clone())
            .collect();

        query!(
            "         
            DELETE FROM abilities 
            WHERE abilities.character_id = $1::INTEGER AND (abilities.name, COALESCE(abilities.subskill, 'NO_SUBSKILL')) 
                IN (SELECT
                    name, COALESCE(subskill, 'NO_SUBSKILL') AS subskill
                FROM UNNEST($2::ABILITYNAME[], $3::VARCHAR(255)[]) as data(name, subskill)
            )",
            character_id,
            &names_to_remove as &[AbilityNamePostgres],
            &subskills_to_remove as &[Option<String>],
        ).execute(&mut *transaction).await.wrap_err("Database error attempting to remove abilities")?;

        Ok(())
    }

    async fn upsert_abilities(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        let (
            mut craft_and_ma_names_to_upsert,
            mut craft_and_ma_subskills_to_upsert,
            mut craft_and_ma_dots_to_upsert,
            mut other_names_to_update,
            mut other_dots_to_update,
        ) = (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());

        for ability_to_upsert in self.abilities_to_upsert.iter() {
            match ability_to_upsert.0 {
                AbilityNamePostgres::Craft => {
                    craft_and_ma_names_to_upsert.push(AbilityNamePostgres::Craft);
                    craft_and_ma_subskills_to_upsert.push(ability_to_upsert.1.as_deref());
                    craft_and_ma_dots_to_upsert.push(ability_to_upsert.2 as i16);
                }
                AbilityNamePostgres::MartialArts => {
                    craft_and_ma_names_to_upsert.push(AbilityNamePostgres::MartialArts);
                    craft_and_ma_subskills_to_upsert.push(ability_to_upsert.1.as_deref());
                    craft_and_ma_dots_to_upsert.push(ability_to_upsert.2 as i16);
                }
                other_name_postgres => {
                    other_names_to_update.push(other_name_postgres);
                    other_dots_to_update.push(ability_to_upsert.2 as i16);
                }
            }
        }

        if !other_names_to_update.is_empty() {
            query!(
                "UPDATE abilities
                SET dots = data.dots
                FROM UNNEST($2::ABILITYNAME[], $3::SMALLINT[]) as data(name, dots)
                WHERE abilities.character_id = $1 AND abilities.name = data.name",
                character_id,
                &other_names_to_update as &[AbilityNamePostgres],
                &other_dots_to_update as &[i16]
            )
            .execute(&mut *transaction)
            .await
            .wrap_err("Database error attempting to update non-Craft, non-MartialArts abilities")?;
        }

        if !craft_and_ma_names_to_upsert.is_empty() {
            query!(
                "INSERT INTO abilities(character_id, name, dots, subskill)
                SELECT 
                    $1::INTEGER as character_id, 
                    name, 
                    dots, 
                    subskill 
                FROM UNNEST($2::ABILITYNAME[], $3::SMALLINT[], $4::VARCHAR(255)[]) as data(name, dots, subskill)
                ON CONFLICT ON CONSTRAINT unique_abilities
                DO UPDATE SET dots = EXCLUDED.dots
                ",
                character_id,
                &craft_and_ma_names_to_upsert as &[AbilityNamePostgres],
                &craft_and_ma_dots_to_upsert as &[i16],
                &craft_and_ma_subskills_to_upsert as &[Option<&str>]
            ).execute(&mut *transaction).await.wrap_err("Database error attempting to upsert Craft and Martial Arts abilities")?;
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
            .map(|(ability_name, _, _)| *ability_name)
            .collect();

        let ability_subskill_with_specialty_to_remove: Vec<Option<&str>> = self
            .specialties_to_remove
            .iter()
            .map(|(_, subskill, _)| subskill.as_ref().map(|s| s.as_str()))
            .collect();

        let specialty_name_to_remove: Vec<&str> = self
            .specialties_to_remove
            .iter()
            .map(|(_, _, text)| text.as_str())
            .collect();

        query!("
            DELETE FROM specialties
            WHERE (specialties.ability_id, specialties.specialty) IN
            (
                SELECT
                    abilities.id,
                    specialties.specialty
                FROM abilities INNER JOIN specialties ON (specialties.ability_id = abilities.id)
                WHERE abilities.character_id = $1::INTEGER
                AND (abilities.name, COALESCE(abilities.subskill, 'NO_SUBSKILL'), specialties.specialty)
                IN (
                    SELECT
                        name,
                        COALESCE(subskill, 'NO SUBSKILL') AS subskill,
                        specialty
                    FROM UNNEST($2::ABILITYNAME[], $3::VARCHAR(255)[], $4::VARCHAR(255)[]) as data(name, subskill, specialty)
                )
            )
        ",
        character_id,
        &ability_name_with_specialty_to_remove as &[AbilityNamePostgres],
        &ability_subskill_with_specialty_to_remove as &[Option<&str>],
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
            .map(|(name, _, _)| *name)
            .collect();

        let ability_subskill_with_specialty_to_add: Vec<Option<&str>> = self
            .specialties_to_add
            .iter()
            .map(|(_, subskill, _)| subskill.as_ref().map(|s| s.as_str()))
            .collect();

        let specialty_name_to_add: Vec<&str> = self
            .specialties_to_add
            .iter()
            .map(|(_, _, text)| text.as_str())
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
                    COALESCE(subskill, 'NO_SUBSKILL') as subskill,
                    specialty
                FROM UNNEST($2::ABILITYNAME[], $3::VARCHAR(255)[], $4::VARCHAR(255)[]) AS data(name, subskill, specialty)
            ) AS added 
            ON (abilities.name = added.name AND COALESCE(abilities.subskill, 'NO_SUBSKILL') = added.subskill)
            WHERE abilities.character_id = $1::INTEGER
            "#,
            character_id as i32,
            &ability_name_with_specialty_to_add as &[AbilityNamePostgres],
            &ability_subskill_with_specialty_to_add as &[Option<&str>],
            &specialty_name_to_add as &[&str],
        ).execute(&mut *transaction).await.wrap_err("Database error attempting to insert specialties")?;

        Ok(())
    }

    pub async fn update(
        self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        if !self.abilities_to_remove.is_empty() {
            self.remove_abilities(transaction, character_id)
                .await
                .wrap_err("Error attempting to remove abilities")?;
        }

        if !self.abilities_to_upsert.is_empty() {
            self.upsert_abilities(transaction, character_id)
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
