use serde::{Deserialize, Serialize};

use crate::weapons::weapon::base::BaseWeaponMemo;

use super::WornMundaneWeapon;

/// A worn mundane weapon like a cestus or razor claws.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WornMundaneWeaponMemo(pub(crate) BaseWeaponMemo);

impl<'source> WornMundaneWeaponMemo {
    pub(crate) fn as_ref(&'source self) -> WornMundaneWeapon<'source> {
        WornMundaneWeapon(&self.0)
    }
}
