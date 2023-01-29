use crate::CharacterMutation;

use super::MundaneArmorName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveMundaneArmor(pub MundaneArmorName);

impl From<RemoveMundaneArmor> for CharacterMutation {
    fn from(remove_mundane_armor: RemoveMundaneArmor) -> Self {
        CharacterMutation::RemoveMundaneArmor(remove_mundane_armor)
    }
}