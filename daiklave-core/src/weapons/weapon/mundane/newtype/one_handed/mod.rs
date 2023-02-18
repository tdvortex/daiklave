mod memo;

use std::ops::Deref;

pub use memo::OneHandedMundaneWeaponMemo;

use crate::weapons::weapon::base::BaseWeapon;

/// A one-handed mundane weapon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OneHandedMundaneWeaponView<'source>(pub(crate) &'source BaseWeapon);

impl<'source> Deref for OneHandedMundaneWeaponView<'source> {
    type Target = BaseWeapon;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'source> From<&'source OneHandedMundaneWeaponMemo> for OneHandedMundaneWeaponView<'source> {
    fn from(value: &'source OneHandedMundaneWeaponMemo) -> Self {
        Self(&value.0)
    }
}
