use std::collections::HashSet;
use eyre::Result;
use sqlx::{Transaction, Postgres};

use crate::{character::traits::weapons::{Weapon, Weapons}};

#[derive(Debug, Default)]
pub struct WeaponsDiff {
    new_weapons: Vec<Weapon>,
    removed_weapons: Vec<i32>,
}

impl Weapons {
    pub fn compare_newer(&self, newer: &Self) -> WeaponsDiff {
        let mut diff = WeaponsDiff::default();

        let old_ids: HashSet<i32> = self.iter().filter_map(|(_, weapon)| weapon.id()).collect();

        newer.iter().for_each(|(_, weapon)| {
            if let Some(id) = weapon.id() {
                if !old_ids.contains(&id) {
                    diff.removed_weapons.push(id);
                }
            } else {
                diff.new_weapons.push(weapon.clone())
            }
        });

        diff
    }
}

impl WeaponsDiff {
    pub async fn save(
        self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        if !self.new_weapons.is_empty() {
            crate::database::queries::post_weapons_transaction(transaction, self.new_weapons).await?;

            query!(
                "
                DELETE FROM character_weapons
                "
            )
        }
    }
}