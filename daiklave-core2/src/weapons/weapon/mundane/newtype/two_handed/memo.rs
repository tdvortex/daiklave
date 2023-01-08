use serde::{Deserialize, Serialize};

use crate::weapons::weapon::base::BaseWeapon;

use super::TwoHandedMundaneWeaponView;

/// A two-handed mundane weapon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwoHandedMundaneWeapon(pub(crate) BaseWeapon);

impl<'source> TwoHandedMundaneWeapon {
    pub(crate) fn as_ref(&'source self) -> TwoHandedMundaneWeaponView<'source> {
        TwoHandedMundaneWeaponView(&self.0)
    }
}
