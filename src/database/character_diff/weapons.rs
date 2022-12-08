use eyre::Result;
use sqlx::{Postgres, Transaction};
use std::collections::HashMap;

use crate::{
    character::traits::weapons::{EquipHand, Weapon, Weapons},
    database::tables::weapons::EquipHandPostgres,
};

#[derive(Debug, Default)]
pub struct WeaponsDiff<'a> {
    insert_weapons: Vec<(&'a Weapon, Option<EquipHand>)>,
    removed_weapons: Vec<i32>,
    equipped_weapons: Vec<(i32, EquipHandPostgres)>,
    unequipped_weapons: Vec<(i32, EquipHandPostgres)>,
}

impl<'a> Weapons {
    pub fn compare_newer(&self, newer: &'a Self) -> WeaponsDiff<'a> {
        let mut diff = WeaponsDiff::default();

        let mut old_weapons = HashMap::<i32, Option<EquipHand>>::new();

        for (_, maybe_equip_hand, weapon) in self.iter() {
            if let Some(id) = weapon.id() {
                old_weapons.insert(id, maybe_equip_hand);
            }
        }

        let mut new_weapons = HashMap::<i32, Option<EquipHand>>::new();

        for (_, maybe_equip_hand, weapon) in newer.iter() {
            if let Some(id) = weapon.id() {
                new_weapons.insert(id, maybe_equip_hand);
            } else {
                diff.insert_weapons.push((weapon, maybe_equip_hand));
            }
        }

        for (key, old_hands) in old_weapons.iter() {
            if let Some(new_hands) = new_weapons.get(key) {
                match (old_hands, new_hands) {
                    (None, None)
                    | (Some(EquipHand::Main), Some(EquipHand::Main))
                    | (Some(EquipHand::Off), Some(EquipHand::Off))
                    | (Some(EquipHand::Both), Some(EquipHand::Both)) => { /* do nothing */ }
                    (None, Some(EquipHand::Main))
                    | (Some(EquipHand::Off), Some(EquipHand::Both)) => {
                        diff.equipped_weapons.push((*key, EquipHandPostgres::Main));
                    }
                    (None, Some(EquipHand::Off))
                    | (Some(EquipHand::Main), Some(EquipHand::Both)) => {
                        diff.equipped_weapons.push((*key, EquipHandPostgres::Off));
                    }
                    (None, Some(EquipHand::Both)) => {
                        diff.equipped_weapons.push((*key, EquipHandPostgres::Main));
                        diff.equipped_weapons.push((*key, EquipHandPostgres::Off));
                    }
                    (Some(EquipHand::Main), None)
                    | (Some(EquipHand::Both), Some(EquipHand::Off)) => {
                        diff.unequipped_weapons
                            .push((*key, EquipHandPostgres::Main));
                    }
                    (Some(EquipHand::Off), None)
                    | (Some(EquipHand::Both), Some(EquipHand::Main)) => {
                        diff.unequipped_weapons.push((*key, EquipHandPostgres::Off));
                    }
                    (Some(EquipHand::Both), None) => {
                        diff.unequipped_weapons
                            .push((*key, EquipHandPostgres::Main));
                        diff.unequipped_weapons.push((*key, EquipHandPostgres::Off));
                    }
                    (Some(EquipHand::Main), Some(EquipHand::Off)) => {
                        diff.unequipped_weapons
                            .push((*key, EquipHandPostgres::Main));
                        diff.equipped_weapons.push((*key, EquipHandPostgres::Off));
                    }
                    (Some(EquipHand::Off), Some(EquipHand::Main)) => {
                        diff.equipped_weapons.push((*key, EquipHandPostgres::Main));
                        diff.unequipped_weapons.push((*key, EquipHandPostgres::Off));
                    }
                }
            } else {
                diff.removed_weapons.push(*key);
            }
        }

        diff
    }
}

impl<'a> WeaponsDiff<'a> {
    pub async fn save(
        self,
        transaction: &mut Transaction<'_, Postgres>,
        character_id: i32,
    ) -> Result<()> {
        todo!()
    }
}
