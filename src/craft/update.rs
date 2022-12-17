use eyre::{Result, WrapErr};
use sqlx::{query, Postgres, Transaction};
use std::collections::HashSet;

use super::CraftAbilities;

#[derive(Debug, Default)]
pub(crate) struct CraftDiff {
    upserted_foci: Vec<(String, u8)>,
    removed_foci: Vec<String>,
    added_specialties: Vec<(String, String)>,
    removed_specialties: Vec<(String, String)>,
}

impl CraftAbilities {
    pub(crate) fn compare_newer(&self, newer: &Self) -> CraftDiff {
        let mut diff = CraftDiff::default();

        for ability in newer.iter() {
            if let Some(old_rating) = self.get_rating(ability.name().subskill().unwrap()) {
                if old_rating.dots() != ability.dots() {
                    diff.upserted_foci.push((
                        ability.name().subskill().unwrap().to_owned(),
                        ability.dots(),
                    ));
                }

                match (old_rating.specialties(), ability.specialties()) {
                    (None, None) => {}
                    (None, Some(added)) => {
                        for specialty in added.iter() {
                            diff.added_specialties.push((
                                ability.name().subskill().unwrap().to_owned(),
                                specialty.clone(),
                            ))
                        }
                    }
                    (Some(removed), None) => {
                        for specialty in removed.iter() {
                            diff.removed_specialties.push((
                                ability.name().subskill().unwrap().to_owned(),
                                specialty.clone(),
                            ))
                        }
                    }
                    (Some(old_specialties), Some(new_specialties)) => {
                        let mut old_specialties_set: HashSet<&str> =
                            old_specialties.iter().map(|s| s.as_str()).collect();

                        for specialty in new_specialties {
                            if !old_specialties_set.remove(specialty.as_str()) {
                                diff.added_specialties.push((
                                    ability.name().subskill().unwrap().to_owned(),
                                    specialty.clone(),
                                ))
                            }
                        }

                        for specialty in old_specialties_set.into_iter() {
                            diff.removed_specialties.push((
                                ability.name().subskill().unwrap().to_owned(),
                                specialty.to_owned(),
                            ))
                        }
                    }
                }
            } else {
                diff.upserted_foci.push((
                    ability.name().subskill().unwrap().to_owned(),
                    ability.dots(),
                ));

                if let Some(specialties) = ability.specialties() {
                    for specialty in specialties {
                        diff.added_specialties.push((
                            ability.name().subskill().unwrap().to_owned(),
                            specialty.clone(),
                        ));
                    }
                }
            }
        }

        for ability in self.iter() {
            if newer
                .get_rating(ability.name().subskill().unwrap())
                .is_none()
            {
                diff.removed_foci
                    .push(ability.name().subskill().unwrap().to_owned());
            }
        }

        diff
    }
}

impl CraftDiff {
    pub async fn update(
        self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        query!(
            "DELETE FROM craft_abilities
            WHERE character_id = $1 AND focus IN (SELECT * FROM UNNEST($2::VARCHAR(255)[]))
            ",
            character_id,
            &self.removed_foci as &[String]
        )
        .execute(&mut *transaction)
        .await
        .wrap_err_with(|| "Database error removing craft foci")?;

        let (upserted_foci, upserted_dots) = self.upserted_foci.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut upserted_foci, mut upserted_dots), (focus, dots)| {
                upserted_foci.push(focus);
                upserted_dots.push(dots);
                (upserted_foci, upserted_dots)
            },
        );

        query!(
            "INSERT INTO craft_abilities(character_id, focus, dots)
            SELECT
                $1 as character_id,
                data.focus as focus,
                data.dots as dots
            FROM UNNEST($2::VARCHAR(255)[], $3::SMALLINT[]) as data(focus, dots)
            ON CONFLICT (character_id, focus) DO UPDATE SET dots = EXCLUDED.dots
            ",
            character_id,
            &upserted_foci as &[String],
            &upserted_dots as &[u8]
        )
        .execute(&mut *transaction)
        .await
        .wrap_err_with(|| "Database error upserting craft foci")?;

        let (foci_with_removed_specialty, removed_specialty) =
            self.removed_specialties.into_iter().fold(
                (Vec::new(), Vec::new()),
                |(mut foci_with_removed_specialty, mut removed_specialty), (focus, specialty)| {
                    foci_with_removed_specialty.push(focus);
                    removed_specialty.push(specialty);
                    (foci_with_removed_specialty, removed_specialty)
                },
            );

        query!(
            "DELETE FROM craft_ability_specialties
            WHERE character_id = $1 AND (focus, specialty) IN 
            (
                SELECT
                    data.focus as focus,
                    data.specialty as specialty
                FROM UNNEST($2::VARCHAR(255)[], $3::VARCHAR(255)[]) as data(focus, specialty)
            )",
            character_id,
            &foci_with_removed_specialty as &[String],
            &removed_specialty as &[String]
        )
        .execute(&mut *transaction)
        .await
        .wrap_err_with(|| "Database error deleting craft specialties")?;

        let (foci_with_added_specialty, added_specialty) = self.added_specialties.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut foci_with_added_specialty, mut added_specialty), (focus, specialty)| {
                foci_with_added_specialty.push(focus);
                added_specialty.push(specialty);
                (foci_with_added_specialty, added_specialty)
            },
        );

        query!(
            "INSERT INTO craft_ability_specialties(character_id, focus, specialty)
            SELECT
                $1 as character_id,
                data.focus as focus,
                data.specialty as specialty
            FROM UNNEST($2::VARCHAR(255)[], $3::VARCHAR(255)[]) as data(focus, specialty)
            ",
            character_id,
            &foci_with_added_specialty as &[String],
            &added_specialty as &[String]
        )
        .execute(&mut *transaction)
        .await
        .wrap_err_with(|| "Database error deleting craft specialties")?;

        Ok(())
    }
}
