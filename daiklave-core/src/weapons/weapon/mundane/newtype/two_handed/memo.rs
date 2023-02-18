use serde::{Deserialize, Serialize};

use crate::weapons::weapon::base::BaseWeapon;

use super::TwoHandedMundaneWeaponView;

/// A two-handed mundane weapon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwoHandedMundaneWeapon(pub(crate) BaseWeapon);

impl From<&TwoHandedMundaneWeaponView<'_>> for TwoHandedMundaneWeapon {
    fn from(value: &TwoHandedMundaneWeaponView<'_>) -> Self {
        Self(value.0.to_owned())
    }
}