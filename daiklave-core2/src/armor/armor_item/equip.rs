use crate::CharacterMutation;

use super::{ArmorNameMutation};

pub struct EquipArmor(ArmorNameMutation);

impl EquipArmor {
    pub fn new(name: ArmorNameMutation) -> Self {
        Self(name)
    }
}

impl From<EquipArmor> for CharacterMutation {
    fn from(equip_armor: EquipArmor) -> Self {
        CharacterMutation::EquipArmor(equip_armor)
    }
}