use crate::CharacterMutation;

use super::{WeaponNameMutation, Equipped};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnequipWeapon {
    pub(crate) name: WeaponNameMutation,
    pub(crate) equipped: Equipped,
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