use crate::CharacterMutation;

use super::{EquipHand, WeaponNameMutation, WeaponName};

/// A mutation to equip a specific weapon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquipWeapon {
    pub(crate) weapon_name: WeaponNameMutation,
    pub(crate) hand: Option<EquipHand>
}

impl EquipWeapon {
    /// Creates a mutation to equip a two-handed weapon.
    pub fn two_handed(name: WeaponName<'_>) -> Self {
        Self {
            weapon_name: name.into(),
            hand: None,
        }
    }

    /// Creates a mutation to equip a worn weapon.
    pub fn worn(name: WeaponName<'_>) -> Self {
        Self {
            weapon_name: name.into(),
            hand: None,
        }
    }

    /// Creates a mutation to equip a one-handed weapon in the main hand.
    pub fn main_hand(name: WeaponName<'_>) -> Self {
        Self {
            weapon_name: name.into(),
            hand: Some(EquipHand::MainHand),
        }
    }

    /// Creates a mutation to equip a one-handed weapon in the off hand.
    pub fn off_hand(name: WeaponName<'_>) -> Self {
        Self {
            weapon_name: name.into(),
            hand: Some(EquipHand::OffHand),
        }
    }
}


impl From<EquipWeapon> for CharacterMutation {
    fn from(equip_weapon: EquipWeapon) -> Self {
        CharacterMutation::EquipWeapon(equip_weapon)
    }
}