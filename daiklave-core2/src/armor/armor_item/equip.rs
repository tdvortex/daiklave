use crate::CharacterMutation;

use super::{ArmorNameMutation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquipArmor(pub ArmorNameMutation);

impl From<EquipArmor> for CharacterMutation {
    fn from(equip_armor: EquipArmor) -> Self {
        CharacterMutation::EquipArmor(equip_armor)
    }
}