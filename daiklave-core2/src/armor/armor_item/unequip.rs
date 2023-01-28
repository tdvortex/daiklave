use crate::CharacterMutation;

pub struct UnequipArmor;

impl From<UnequipArmor> for CharacterMutation {
    fn from(unequip_armor: UnequipArmor) -> Self {
        CharacterMutation::UnequipArmor
    }
}