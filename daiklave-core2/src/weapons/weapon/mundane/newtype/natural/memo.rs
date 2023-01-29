use serde::{Deserialize, Serialize};

use crate::weapons::weapon::base::BaseWeapon;

/// A mundane weapon that is part of the user's body, like Unarmed
/// or shapeshifted claws.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NaturalMundaneWeapon(pub(crate) BaseWeapon);