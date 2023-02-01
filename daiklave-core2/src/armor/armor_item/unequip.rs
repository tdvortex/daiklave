use crate::CharacterMutation;

/// A mutation to unequip whatever armor the character is currently wearing.
pub struct UnequipArmor;

impl From<UnequipArmor> for CharacterMutation {
    fn from(_: UnequipArmor) -> Self {
        CharacterMutation::UnequipArmor
    }
}