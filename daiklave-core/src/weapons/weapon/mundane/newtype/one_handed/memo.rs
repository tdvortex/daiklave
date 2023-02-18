use serde::{Deserialize, Serialize};

use crate::weapons::weapon::base::BaseWeapon;

use super::OneHandedMundaneWeaponView;
/// A one-handed mundane weapon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OneHandedMundaneWeaponMemo(pub(crate) BaseWeapon);

impl From<&OneHandedMundaneWeaponView<'_>> for OneHandedMundaneWeaponMemo {
    fn from(one: &OneHandedMundaneWeaponView<'_>) -> Self {
        Self(one.0.to_owned())
    }
}