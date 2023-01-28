use crate::CharacterMutation;

use super::MundaneArmorName;

pub struct RemoveMundaneArmor(MundaneArmorName);

impl RemoveMundaneArmor {
    pub fn new(name: MundaneArmorName) -> Self {
        Self(name)
    }
}

impl From<RemoveMundaneArmor> for CharacterMutation {
    fn from(remove_mundane_armor: RemoveMundaneArmor) -> Self {
        CharacterMutation::RemoveMundaneArmor(remove_mundane_armor)
    }
}