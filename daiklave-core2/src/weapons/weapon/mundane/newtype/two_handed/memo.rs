use serde::{Deserialize, Serialize};

use crate::weapons::weapon::base::BaseWeapon;

use super::TwoHandedMundaneWeapon;

/// A two-handed mundane weapon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwoHandedMundaneWeaponMemo(pub(crate) BaseWeapon);

impl<'source> TwoHandedMundaneWeaponMemo {
    pub(crate) fn as_ref(&'source self) -> TwoHandedMundaneWeapon<'source> {
        TwoHandedMundaneWeapon(&self.0)
    }
}
