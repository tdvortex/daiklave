mod memo;

use std::ops::Deref;

use crate::weapons::weapon::base::BaseWeapon;

pub use self::memo::TwoHandedMundaneWeapon;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TwoHandedMundaneWeaponView<'source>(pub(crate) &'source BaseWeapon);

impl<'source> Deref for TwoHandedMundaneWeaponView<'source> {
    type Target = BaseWeapon;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'source> From<&'source TwoHandedMundaneWeapon> for TwoHandedMundaneWeaponView<'source> {
    fn from(value: &'source TwoHandedMundaneWeapon) -> Self {
        Self(&value.0)
    }
}
