use std::ops::Deref;

use serde::{Serialize, Deserialize};

use crate::weapons::{weapon::{EquipWeapon, WeaponName, UnequipWeapon, Equipped}, WeaponError};

use super::RemoveMundaneWeapon;

/// The name of a mundane weapon.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct MundaneWeaponName(String);

impl MundaneWeaponName {
    /// Creates a mutation to equip this weapon as a two-handed weapon.
    pub fn equip_two_handed(&self) -> EquipWeapon {
        EquipWeapon::two_handed(WeaponName::Mundane(self.as_str()))
    }

    /// Creates a mutation to equip this weapon as a worn weapon.
    pub fn equip_worn(&self) -> EquipWeapon {
        EquipWeapon::worn(WeaponName::Mundane(self.as_str()))
    }

    /// Creates a mutation to equip this weapon in the main hand.
    pub fn equip_main_hand(&self) -> EquipWeapon {
        EquipWeapon::main_hand(WeaponName::Mundane(self.as_str()))
    }

    /// Creates a mutation to equip this weapon in the off hand.
    pub fn equip_off_hand(&self) -> EquipWeapon {
        EquipWeapon::off_hand(WeaponName::Mundane(self.as_str()))
    }

    /// Creates a mutation to unequip a weapon. Returns an Err if trying to
    /// unequip a worn weapon.
    pub fn unequip(&self, equipped: Equipped) -> Result<UnequipWeapon, WeaponError> {
        UnequipWeapon::new(WeaponName::Mundane(self.as_str()), equipped)
    }

    /// Creates a mutation to remove a mundane weapon. Defaults to 1, but can
    /// be modified.
    pub fn remove(&self) -> RemoveMundaneWeapon {
        RemoveMundaneWeapon::name(self.clone())
    }
}

impl<T> From<T> for MundaneWeaponName where T: Into<String> {
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for MundaneWeaponName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}