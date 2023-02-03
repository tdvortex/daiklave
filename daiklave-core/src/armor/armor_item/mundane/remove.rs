use crate::CharacterMutation;

use super::MundaneArmorName;

/// A mutation to remove a piece of mundane armor from a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveMundaneArmor(pub MundaneArmorName);

impl RemoveMundaneArmor {
    /// Constructs a RemoveMundaneArmor mutation.
    pub fn name(name: impl Into<MundaneArmorName>) -> Self {
        Self(name.into())
    }
}

impl From<MundaneArmorName> for RemoveMundaneArmor {
    fn from(name: MundaneArmorName) -> Self {
        Self(name)
    }
}

impl From<RemoveMundaneArmor> for CharacterMutation {
    fn from(remove_mundane_armor: RemoveMundaneArmor) -> Self {
        CharacterMutation::RemoveMundaneArmor(remove_mundane_armor)
    }
}
