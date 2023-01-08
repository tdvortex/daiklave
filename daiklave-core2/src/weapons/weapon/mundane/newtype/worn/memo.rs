use serde::{Deserialize, Serialize};

use crate::weapons::weapon::base::BaseWeapon;

use super::WornMundaneWeaponView;

/// A worn mundane weapon like a cestus or razor claws.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WornMundaneWeapon(pub(crate) BaseWeapon);

impl<'source> WornMundaneWeapon {
    pub(crate) fn as_ref(&'source self) -> WornMundaneWeaponView<'source> {
        WornMundaneWeaponView(&self.0)
    }
}
