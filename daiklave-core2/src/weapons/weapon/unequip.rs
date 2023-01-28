use crate::CharacterMutation;

use super::{WeaponNameMutation, Equipped};

pub struct UnequipWeapon {
    name: WeaponNameMutation,
    equipped: Equipped,
}

impl UnequipWeapon {
    pub fn new(name: WeaponNameMutation, equipped: Equipped) -> Self {
        Self {
            name,
            equipped,
        }
    }
}

impl From<UnequipWeapon> for CharacterMutation {
    fn from(unequip_weapon: UnequipWeapon) -> Self {
        CharacterMutation::UnequipWeapon(unequip_weapon)
    }
}