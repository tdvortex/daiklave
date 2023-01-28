use crate::CharacterMutation;

use super::{EquipHand, WeaponNameMutation};

pub struct EquipWeapon {
    weapon_name: WeaponNameMutation,
    hand: Option<EquipHand>
}

impl EquipWeapon {
    pub fn new(weapon_name: WeaponNameMutation, hand: Option<EquipHand>) -> Self {
        Self {
            weapon_name,
            hand,
        }
    }
}

impl From<EquipWeapon> for CharacterMutation {
    fn from(equip_weapon: EquipWeapon) -> Self {
        CharacterMutation::EquipWeapon(equip_weapon)
    }
}