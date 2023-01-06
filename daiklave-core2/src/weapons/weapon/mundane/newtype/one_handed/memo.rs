use serde::{Deserialize, Serialize};

use crate::weapons::weapon::base::BaseWeaponMemo;

use super::OneHandedMundaneWeapon;

/// A one-handed mundane weapon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OneHandedMundaneWeaponMemo(pub(crate) BaseWeaponMemo);

impl<'source> OneHandedMundaneWeaponMemo {
    pub(crate) fn as_ref(&'source self) -> OneHandedMundaneWeapon<'source> {
        OneHandedMundaneWeapon(&self.0)
    }
}
