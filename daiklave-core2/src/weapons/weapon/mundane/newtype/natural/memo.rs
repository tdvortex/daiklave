use serde::{Deserialize, Serialize};

use crate::weapons::weapon::base::BaseWeapon;

use super::NaturalMundaneWeapon;

/// A mundane weapon that is part of the user's body, like Unarmed
/// or shapeshifted claws.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NaturalMundaneWeaponMemo(pub(crate) BaseWeapon);

impl<'source> NaturalMundaneWeaponMemo {
    pub(crate) fn as_ref(&'source self) -> NaturalMundaneWeapon<'source> {
        NaturalMundaneWeapon(&self.0)
    }
}
