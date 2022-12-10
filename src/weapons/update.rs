use std::collections::HashMap;

use eyre::Result;
use sqlx::{query, Postgres, Transaction};

use crate::weapons::create::create_weapons_transaction;
use crate::weapons::tables::EquipHandPostgres;
use crate::weapons::{EquipHand, Weapon, Weapons};

#[derive(Debug, Default)]
pub struct WeaponsDiff {
    noop: bool,
    created_weapons: Vec<(Weapon, Option<EquipHand>)>,
    owned_weapons: Vec<(i32, Option<EquipHand>)>,
}

impl Weapons {
    pub fn compare_newer(&self, newer: &Self) -> WeaponsDiff {
        let mut noop = true;
        let mut created_weapons = Vec::new();
        let mut new_owned_weapons = HashMap::new();

        for (_, maybe_equip_hand, weapon) in newer.iter() {
            if let Some(id) = weapon.id() {
                new_owned_weapons.insert(id, maybe_equip_hand);
            } else {
                noop = false;
                created_weapons.push((weapon.clone(), maybe_equip_hand));
            }
        }

        if noop {
            let mut old_owned_weapons = HashMap::new();

            for (_, maybe_equip_hand, weapon) in self.iter() {
                if let Some(id) = weapon.id() {
                    old_owned_weapons.insert(id, maybe_equip_hand);
                }
            }

            noop = old_owned_weapons != new_owned_weapons;
        }

        if noop {
            WeaponsDiff {
                noop,
                created_weapons: Vec::new(),
                owned_weapons: Vec::new(),
            }
        } else {
            WeaponsDiff {
                noop,
                created_weapons,
                owned_weapons: new_owned_weapons.into_iter().collect(),
            }
        }
    }
}

impl WeaponsDiff {
    pub async fn update(
        self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        if self.noop {
            return Ok(());
        }

        // Drop all owned/equipped records
        query!(
            "DELETE FROM character_weapons
            WHERE character_id = $1",
            character_id
        )
        .execute(&mut *transaction)
        .await?;

        let (hands, weapons) = self.created_weapons.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut hands, mut weapons), (weapon, maybe_hand)| {
                hands.push(maybe_hand);
                weapons.push(weapon);
                (hands, weapons)
            },
        );

        let created_ids = create_weapons_transaction(transaction, weapons).await?;

        let (ids, hands_postgres) = created_ids
            .into_iter()
            .zip(hands.into_iter())
            .chain(self.owned_weapons.into_iter())
            .fold(
                (Vec::new(), Vec::new()),
                |(mut ids, mut hands_postgres), (id, maybe_hand)| {
                    match maybe_hand {
                        Some(EquipHand::Both) => {
                            hands_postgres.push(Some(EquipHandPostgres::Main));
                            hands_postgres.push(Some(EquipHandPostgres::Off));
                            ids.push(id);
                            ids.push(id);
                        }
                        Some(EquipHand::Main) => {
                            hands_postgres.push(Some(EquipHandPostgres::Main));
                            ids.push(id);
                        }
                        Some(EquipHand::Off) => {
                            hands_postgres.push(Some(EquipHandPostgres::Off));
                            ids.push(id);
                        }
                        None => {
                            hands_postgres.push(None);
                            ids.push(id);
                        }
                    };
                    (ids, hands_postgres)
                },
            );

        query!(
            "INSERT INTO character_weapons(character_id, weapon_id, equip_hand)
            SELECT
                $1::INTEGER as character_id,
                data.id as weapon_id,
                data.hand as equip_hand
            FROM UNNEST($2::INTEGER[], $3::EQUIPHAND[]) as data(id, hand)
            ",
            character_id,
            &ids as &[i32],
            &hands_postgres as &[Option<EquipHandPostgres>],
        )
        .execute(&mut *transaction)
        .await?;

        Ok(())
    }
}
