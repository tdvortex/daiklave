use std::num::NonZeroU8;

use crate::CharacterMutation;

use super::MundaneWeaponName;

/// A mutation to remove a specific quantity of a mundane weapon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveMundaneWeapon {
    /// The name of the weapon to remove.
    pub name: MundaneWeaponName,
    /// The quantity to remove.
    pub quantity: NonZeroU8,
}

impl RemoveMundaneWeapon {
    /// Creates a mutation to remove this weapon.
    pub fn name(name: impl Into<MundaneWeaponName>) -> Self {
        Self {
            name: name.into(),
            quantity: NonZeroU8::new(1).unwrap(),
        }
    }

    /// Updates the quantity to remove.
    pub fn quantity(mut self, quantity: NonZeroU8) -> Self {
        self.quantity = quantity;
        self
    }
}

impl From<RemoveMundaneWeapon> for CharacterMutation {
    fn from(remove_mundane_weapon: RemoveMundaneWeapon) -> Self {
        CharacterMutation::RemoveMundaneWeapon(remove_mundane_weapon)
    }
}