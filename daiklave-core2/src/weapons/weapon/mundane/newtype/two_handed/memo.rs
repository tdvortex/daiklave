use serde::{Deserialize, Serialize};

use crate::weapons::weapon::base::BaseWeapon;

/// A two-handed mundane weapon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwoHandedMundaneWeapon(pub(crate) BaseWeapon);
