use serde::{Deserialize, Serialize};

use crate::weapons::weapon::base::BaseWeapon;
/// A one-handed mundane weapon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OneHandedMundaneWeaponMemo(pub(crate) BaseWeapon);