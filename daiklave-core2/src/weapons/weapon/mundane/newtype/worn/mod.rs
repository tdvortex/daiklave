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