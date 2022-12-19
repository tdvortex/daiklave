use std::collections::HashMap;

use crate::id::Id;
use crate::weapons::{EquipHand, Weapon, Weapons};

#[derive(Debug, Default)]
pub struct WeaponsDiff {
    pub noop: bool,
    pub created_weapons: Vec<(Weapon, Option<EquipHand>)>,
    pub owned_weapons: Vec<(i32, Option<EquipHand>)>,
}

impl Weapons {
    pub fn compare_newer(&self, newer: &Self) -> WeaponsDiff {
        let mut noop = true;
        let mut created_weapons = Vec::new();
        let mut new_owned_weapons = HashMap::new();

        for (_, weapon, maybe_equip_hand) in newer.iter() {
            if let Id::Database(id) = weapon.id() {
                new_owned_weapons.insert(id, maybe_equip_hand);
            } else {
                noop = false;
                created_weapons.push((weapon.clone(), maybe_equip_hand));
            }
        }

        if noop {
            let mut old_owned_weapons = HashMap::new();

            for (_, weapon, maybe_equip_hand) in self.iter() {
                if let Id::Database(id) = weapon.id() {
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
