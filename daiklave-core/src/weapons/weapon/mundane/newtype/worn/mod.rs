mod memo;

use std::ops::Deref;

use crate::weapons::weapon::base::BaseWeapon;

pub use self::memo::WornMundaneWeapon;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WornMundaneWeaponView<'source>(pub(crate) &'source BaseWeapon);

impl<'source> Deref for WornMundaneWeaponView<'source> {
    type Target = BaseWeapon;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'source> From<&'source WornMundaneWeapon> for WornMundaneWeaponView<'source> {
    fn from(value: &'source WornMundaneWeapon) -> Self {
        Self(&value.0)
    }
}