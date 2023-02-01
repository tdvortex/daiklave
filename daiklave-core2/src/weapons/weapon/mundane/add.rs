use std::{num::NonZeroU8};

use crate::{weapons::weapon::{builder::base::{BaseWeaponBuilder, MundaneWeaponBuilder}}, CharacterMutation};

use super::{MundaneWeapon, MundaneWeaponName};

/// A Mundane weapon and its name, to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddMundaneWeapon {
    pub(crate) name: MundaneWeaponName,
    pub(crate) weapon: MundaneWeapon,
    /// The number of copies of this weapon to add. Defaults to 1.
    pub quantity: NonZeroU8,
}

impl AddMundaneWeapon {
    /// Starts constructing a new mundane weapon to add with the given name.
    pub fn name(name: impl Into<String>) -> MundaneWeaponBuilder {
        BaseWeaponBuilder::name(name).mundane()
    }

    /// Changes the quantity of the weapon to add.
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