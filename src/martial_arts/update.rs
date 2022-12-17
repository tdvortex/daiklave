use std::collections::{HashMap};
use eyre::{WrapErr, Result};
use sqlx::{Transaction, Postgres, query};

use crate::{abilities::Ability, charms::MartialArtsCharm, id::Id};

use super::{MartialArtistTraits, MartialArtsStyle};

pub struct MartialArtsDiff {
    removed_styles: Vec<Id>,
    added_styles: Vec<(
        MartialArtsStyle,
        u8,
        Option<Vec<String>>,
        Vec<MartialArtsCharm>,
    )>,
    modified_style: Vec<(Id, u8, Option<Vec<String>>, Vec<MartialArtsCharm>)>,
}

impl MartialArtistTraits {
    pub fn compare_newer(&self, newer: &MartialArtistTraits) -> MartialArtsDiff {
        let mut diff = MartialArtsDiff {
            removed_styles: Vec::new(),
            added_styles: Vec::new(),
            modified_style: Vec::new(),
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
                    ability.specialties().map(|v| v.clone()),
                    vec_ptr.clone(),
                ));
            } else {
                let (_, old_ability, old_vec_ptr) = old_hashmap.remove(&style_ptr.id()).unwrap();

                if ability.dots() != old_ability.dots()
                    || ability.specialties() != old_ability.specialties()
                    || vec_ptr != old_vec_ptr
                {
                    diff.modified_style.push((
                        style_ptr.id(),
                        ability.dots(),
                        ability.specialties().map(|v| v.clone()),
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

impl MartialArtsDiff {
    async fn remove_character_styles(
        self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32
    ) -> Result<()> {
        let removed_ids: Vec<i32> = self.removed_styles.iter().filter_map(|id| if !id.is_placeholder() {Some(**id)} else {None}).collect();

        query!(
            "DELETE FROM character_martial_arts
            WHERE character_id = $1 AND style_id IN (SELECT data.style_id FROM UNNEST($2::INTEGER[]) as data(style_id))",
            character_id as i32,
            &removed_ids as &[i32]
        ).execute(&mut *transaction).await.wrap_err_with(|| format!("Database error removing martial arts styles from character {}", character_id))?;
        Ok(())
    }
}

