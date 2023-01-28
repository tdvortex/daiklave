use std::num::NonZeroU8;

use crate::CharacterMutation;

use super::MundaneWeaponName;

pub struct RemoveMundaneWeapon {
    name: MundaneWeaponName,
    quantity: NonZeroU8,
}

impl RemoveMundaneWeapon {
    pub fn new(name: impl ToString, quantity: NonZeroU8) -> Self {
        Self {
            name: name.into(),
            quantity,
        }
    }
}

impl From<RemoveMundaneWeapon> for CharacterMutation {
    fn from(remove_mundane_weapon: RemoveMundaneWeapon) -> Self {
        CharacterMutation::RemoveMundaneWeapon(remove_mundane_weapon)
    }
}