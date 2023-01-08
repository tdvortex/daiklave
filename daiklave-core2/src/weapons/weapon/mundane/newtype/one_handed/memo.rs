use serde::{Deserialize, Serialize};

use crate::weapons::weapon::base::BaseWeapon;

use super::OneHandedMundaneWeaponView;

/// A one-handed mundane weapon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OneHandedMundaneWeapon(pub(crate) BaseWeapon);

impl<'source> OneHandedMundaneWeapon {
    pub(crate) fn as_ref(&'source self) -> OneHandedMundaneWeaponView<'source> {
        OneHandedMundaneWeaponView(&self.0)
    }
}
