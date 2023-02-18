use serde::{Deserialize, Serialize};

use crate::weapons::weapon::base::BaseWeapon;

use super::WornMundaneWeaponView;

/// A worn mundane weapon like a cestus or razor claws.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WornMundaneWeapon(pub(crate) BaseWeapon);

impl From<&WornMundaneWeaponView<'_>> for WornMundaneWeapon {
    fn from(view: &WornMundaneWeaponView<'_>) -> Self {
        Self(view.0.to_owned())
    }
}