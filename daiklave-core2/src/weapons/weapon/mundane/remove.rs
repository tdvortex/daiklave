use std::num::NonZeroU8;

use crate::CharacterMutation;

use super::MundaneWeaponName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveMundaneWeapon {
    pub name: MundaneWeaponName,
    pub quantity: NonZeroU8,
}

impl From<RemoveMundaneWeapon> for CharacterMutation {
    fn from(remove_mundane_weapon: RemoveMundaneWeapon) -> Self {
        CharacterMutation::RemoveMundaneWeapon(remove_mundane_weapon)
    }
}