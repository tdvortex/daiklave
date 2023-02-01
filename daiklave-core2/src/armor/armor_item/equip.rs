use crate::CharacterMutation;

use super::{ArmorNameMutation};

/// A mutation to equip a specific piece of armor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquipArmor(pub(crate) ArmorNameMutation);



impl From<EquipArmor> for CharacterMutation {
    fn from(equip_armor: EquipArmor) -> Self {
        CharacterMutation::EquipArmor(equip_armor)
    }
}