use eyre::{eyre, Result, WrapErr};
use sqlx::{query, Postgres, Transaction};
use std::collections::HashMap;

use crate::{abilities::Ability, charms::MartialArtsCharm, data_source::DataSource, id::Id};

use super::{
    create::{create_martial_arts_charm_transaction, create_martial_arts_style_transaction},
    MartialArtistTraits, MartialArtsStyle,
};

type AddedStyle = (
    MartialArtsStyle,
    u8,
    Option<Vec<String>>,
    Vec<MartialArtsCharm>,
);
type ModifiedStyle = (Id, u8, Option<Vec<String>>, Vec<MartialArtsCharm>);

pub struct MartialArtsDiff {
    removed_styles: Vec<Id>,
    added_styles: Vec<AddedStyle>,
    modified_styles: Vec<ModifiedStyle>,
}

impl MartialArtistTraits {
    pub fn compare_newer(&self, newer: &MartialArtistTraits) -> MartialArtsDiff {
        let mut diff = MartialArtsDiff {
            removed_styles: Vec::new(),
            added_styles: Vec::new(),
            modified_styles: Vec::new(),
        };
        let mut old_hashmap: HashMap<Id, (&MartialArtsStyle, Ability, &Vec<MartialArtsCharm>)> =
            self.iter()
                .map(|(style_ptr, ability, vec_ptr)| {
                    (style_ptr.id(), (style_ptr, ability, vec_ptr))
                })
                .collect();

        for (style_ptr, ability, vec_ptr) in newer.iter() {
            if !old_hashmap.contains_key(&style_ptr.id()) {
                diff.added_styles.push((
                    style_ptr.clone(),
                    ability.dots(),
                    ability.specialties().cloned(),
                    vec_ptr.clone(),
                ));
            } else {
                let (_, old_ability, old_vec_ptr) = old_hashmap.remove(&style_ptr.id()).unwrap();

                if ability.dots() != old_ability.dots()
                    || ability.specialties() != old_ability.specialties()
                    || vec_ptr != old_vec_ptr
                {
                    diff.modified_styles.push((
                        style_ptr.id(),
                        ability.dots(),
                        ability.specialties().cloned(),
                        vec_ptr.clone(),
                    ));
                }
            }
        }

        for id in old_hashmap.keys() {
            diff.removed_styles.push(*id);
        }

        diff
    }
}

async fn upsert_character_styles(
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
    style_database_id: i32,
    style_dots: u8,
    maybe_specialties: Option<&Vec<String>>,
) -> Result<()> {
    query!(
        "INSERT INTO character_martial_arts(character_id, style_id, dots)
        VALUES ($1::INTEGER, $2::INTEGER, $3::SMALLINT)
        ON CONFLICT (character_id, style_id) DO UPDATE
        SET dots = EXCLUDED.dots;",
        character_id as i32,
        style_database_id as i32,
        style_dots as i16
    )
    .execute(&mut *transaction)
    .await
    .wrap_err_with(|| {
        format!(
            "Database error upserting martial arts style dots from character {} for style {}",
            character_id, style_database_id
        )
    })?;

    query!(
        "DELETE FROM character_martial_arts_specialties
        WHERE character_id = $1 AND style_id = $2",
        character_id as i32,
        style_database_id as i32,
    )
    .execute(&mut *transaction)
    .await
    .wrap_err_with(|| {
        format!(
            "Database error deleting martial arts specialties from character {} for style {}",
            character_id, style_database_id
        )
    })?;

    if let Some(specialties) = maybe_specialties {
        let unpacked: Vec<&str> = specialties.iter().map(|s| s.as_str()).collect();

        query!(
            "INSERT INTO character_martial_arts_specialties(character_id, style_id, specialty)
            SELECT
                $1::INTEGER as character_id,
                $2::INTEGER as style_id,
                data.specialty as specialty
            FROM UNNEST($3::VARCHAR(255)[]) as data(specialty)",
            character_id as i32,
            style_database_id as i32,
            &unpacked as &[&str]
        )
        .execute(&mut *transaction)
        .await
        .wrap_err_with(|| {
            format!(
                "Database error inserting martial arts specialties from character {} for style {}",
                character_id, style_database_id
            )
        })?;
    }

    Ok(())
}

async fn upsert_character_charms(
    transaction: &mut Transaction<'_, Postgres>,
    character_id: i32,
    style_charms: &[MartialArtsCharm],
) -> Result<()> {
    let mut charm_database_ids: Vec<i32> = Vec::new();

    for charm in style_charms.iter() {
        if let Id::Database(id) = charm.id() {
            charm_database_ids.push(id);
        } else {
            let id = if let DataSource::Custom(_) = charm.data_source() {
                create_martial_arts_charm_transaction(transaction, charm, Some(character_id))
                    .await?
            } else {
                create_martial_arts_charm_transaction(transaction, charm, None).await?
            };
            charm_database_ids.push(id)
        }
    }

    query!(
        "INSERT INTO character_martial_arts_charms(character_id, charm_id)
        SELECT
            $1::INTEGER as character_id,
            data.charm_id as charm_id
        FROM UNNEST($2::INTEGER[]) as data(charm_id)",
        character_id,
        &charm_database_ids as &[i32]
    )
    .execute(&mut *transaction)
    .await
    .wrap_err_with(|| {
        format!(
            "Database error adding martial arts charms for character {}",
            character_id
        )
    })?;

    Ok(())
}

impl MartialArtsDiff {
    async fn remove_character_styles(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        let removed_ids: Vec<i32> = self
            .removed_styles
            .iter()
            .filter_map(|id| {
                if !id.is_placeholder() {
                    Some(**id)
                } else {
                    None
                }
            })
            .collect();

        if !removed_ids.is_empty() {
            query!(
                "DELETE FROM character_martial_arts
                WHERE character_id = $1 AND style_id IN (SELECT data.style_id FROM UNNEST($2::INTEGER[]) as data(style_id))",
                character_id as i32,
                &removed_ids as &[i32]
            ).execute(&mut *transaction).await.wrap_err_with(|| format!("Database error removing martial arts styles from character {}", character_id))?;
        }
        Ok(())
    }

    async fn add_styles_to_character(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        for (style, style_dots, maybe_specialties, style_charms) in self.added_styles.iter() {
            let style_database_id = if let Id::Database(id) = style.id() {
                id
            } else if let DataSource::Custom(_) = style.data_source() {
                create_martial_arts_style_transaction(transaction, style, Some(character_id))
                    .await?
            } else {
                create_martial_arts_style_transaction(transaction, style, None).await?
            };

            upsert_character_styles(
                transaction,
                character_id,
                style_database_id,
                *style_dots,
                maybe_specialties.as_ref(),
            )
            .await
            .wrap_err("Error attemping to upsert character styles")?;

            if !style_charms.is_empty() {
                upsert_character_charms(transaction, character_id, style_charms).await?;
            }
        }

        Ok(())
    }

    async fn update_character_styles(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        for (style_id, style_dots, maybe_specialties, style_charms) in self.modified_styles.iter() {
            if style_id.is_placeholder() {
                return Err(eyre!("Cannot update a style with a placeholder value"));
            }

            upsert_character_styles(
                transaction,
                character_id,
                **style_id,
                *style_dots,
                maybe_specialties.as_ref(),
            )
            .await?;

            query!(
                "DELETE FROM character_martial_arts_charms
                WHERE character_id = $1",
                character_id
            )
            .execute(&mut *transaction)
            .await
            .wrap_err_with(|| {
                format!(
                    "Database error removing martial arts charms for character {}",
                    character_id
                )
            })?;

            if !style_charms.is_empty() {
                upsert_character_charms(transaction, character_id, style_charms).await?;
            }
        }

        Ok(())
    }

    pub async fn update(
        self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        self.remove_character_styles(transaction, character_id)
            .await
            .wrap_err("Error removing character martial arts styles")?;
        self.add_styles_to_character(transaction, character_id)
            .await
            .wrap_err("Error adding character martial arts styles")?;
        self.update_character_styles(transaction, character_id)
            .await
            .wrap_err("Error updating character martial arts styles")?;
        Ok(())
    }
}
