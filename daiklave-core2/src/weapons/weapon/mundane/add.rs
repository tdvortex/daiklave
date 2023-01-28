use std::{num::NonZeroU8, collections::HashSet};

use crate::{weapons::weapon::{builder::base::BaseWeaponBuilder, range::WeaponRange}, CharacterMutation};

use super::{MundaneWeapon, MundaneWeaponName};

/// A Mundane weapon and its name, to be added to a character.
pub struct AddMundaneWeapon {
    name: MundaneWeaponName,
    weapon: MundaneWeapon,
    pub quantity: NonZeroU8,
}

impl AddMundaneWeapon {
    pub fn builder(name: impl ToString) -> BaseWeaponBuilder {
        BaseWeaponBuilder {
            name: name.to_string(),
            book_reference: None,
            attack_range: WeaponRange::ContactOnly,
            tags: HashSet::new(),
        }
    }
}

impl From<AddMundaneWeapon> for CharacterMutation {
    fn from(add_mundane_weapon: AddMundaneWeapon) -> Self {
        CharacterMutation::AddMundaneWeapon(add_mundane_weapon)
    }
}