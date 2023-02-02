use crate::{weapons::WeaponError, CharacterMutation};

use super::{Equipped, WeaponName, WeaponNameMutation};

/// A mutation to unequip a particular weapon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnequipWeapon {
    pub(crate) name: WeaponNameMutation,
    pub(crate) equipped: Equipped,
}

impl UnequipWeapon {
    /// Creates a new UnequipWeapon mutation. Returns Err if trying to unequip
    /// a Natural weapon.
    pub fn new(name: WeaponName<'_>, equipped: Equipped) -> Result<Self, WeaponError> {
        if equipped == Equipped::Natural {
            Err(WeaponError::UnequipNatural)
        } else {
            Ok(Self {
                name: name.into(),
                equipped,
            })
        }
    }
}

impl From<UnequipWeapon> for CharacterMutation {
    fn from(unequip_weapon: UnequipWeapon) -> Self {
        CharacterMutation::UnequipWeapon(unequip_weapon)
    }
}
