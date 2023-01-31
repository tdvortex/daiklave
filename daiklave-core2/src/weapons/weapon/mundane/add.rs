use std::{num::NonZeroU8, collections::HashSet};

use crate::{weapons::weapon::{builder::base::BaseWeaponBuilder, range::WeaponRange}, CharacterMutation};

use super::{MundaneWeapon, MundaneWeaponName};

/// A Mundane weapon and its name, to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddMundaneWeapon {
    pub(crate) name: MundaneWeaponName,
    pub(crate) weapon: MundaneWeapon,
    pub quantity: NonZeroU8,
}

impl AddMundaneWeapon {
    pub fn builder(name: impl Into<String>) -> BaseWeaponBuilder {
        BaseWeaponBuilder {
            name: name.into(),
            book_reference: None,
            attack_range: WeaponRange::ContactOnly,
            tags: HashSet::new(),
        }
    }

    pub fn set_quantity(&mut self, quantity: NonZeroU8) -> &mut Self {
        self.quantity = quantity;
        self
    }
}

impl From<AddMundaneWeapon> for CharacterMutation {
    fn from(add_mundane_weapon: AddMundaneWeapon) -> Self {
        CharacterMutation::AddMundaneWeapon(add_mundane_weapon)
    }
}