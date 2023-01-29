use crate::CharacterMutation;

use super::{EquipHand, WeaponNameMutation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquipWeapon {
    pub weapon_name: WeaponNameMutation,
    pub hand: Option<EquipHand>
}
impl From<EquipWeapon> for CharacterMutation {
    fn from(equip_weapon: EquipWeapon) -> Self {
        CharacterMutation::EquipWeapon(equip_weapon)
    }
}