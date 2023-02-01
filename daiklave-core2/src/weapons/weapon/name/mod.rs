mod mutation;
pub(crate) use mutation::WeaponNameMutation;

use crate::weapons::WeaponError;

use super::{Equipped, UnequipWeapon, EquipWeapon};

/// The name of a weapon.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum WeaponName<'source> {
    /// All characters have the Unarmed weapon for free, and it cannot
    /// be removed.
    Unarmed,
    /// A mundane weapon without artifact traits.
    Mundane(&'source str),
    /// A unique magical weapon.
    Artifact(&'source str),
}

impl<'source> WeaponName<'source> {
    /// Creates a mutation to equip this weapon as a two-handed weapon.
    pub fn equip_two_handed(self) -> EquipWeapon {
        EquipWeapon::two_handed(self)
    }

    /// Creates a mutation to equip this weapon as a worn weapon.
    pub fn equip_worn(self) -> EquipWeapon {
        EquipWeapon::worn(self)
    }

    /// Creates a mutation to equip this weapon in the main hand.
    pub fn equip_main_hand(self) -> EquipWeapon {
        EquipWeapon::main_hand(self)
    }

    /// Creates a mutation to equip this weapon in the off hand.
    pub fn equip_off_hand(self) -> EquipWeapon {
        EquipWeapon::off_hand(self)
    }

    /// Creates a mutation to unequip a weapon. Returns an Err if trying to
    /// unequip a worn weapon.
    pub fn unequip(self, equipped: Equipped) -> Result<UnequipWeapon, WeaponError> {
        UnequipWeapon::new(self, equipped)
    }
}