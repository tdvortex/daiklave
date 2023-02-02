use serde::{Deserialize, Serialize};

use crate::weapons::weapon::base::BaseWeapon;

/// A worn mundane weapon like a cestus or razor claws.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WornMundaneWeapon(pub(crate) BaseWeapon);
