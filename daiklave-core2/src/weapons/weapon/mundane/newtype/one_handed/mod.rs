mod memo;

use std::ops::Deref;

pub use memo::OneHandedMundaneWeaponMemo;

use crate::weapons::weapon::base::BaseWeapon;

/// A one-handed mundane weapon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneHandedMundaneWeapon<'source>(pub(crate) &'source BaseWeapon);

impl<'source> Deref for OneHandedMundaneWeapon<'source> {
    type Target = BaseWeapon;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'source> OneHandedMundaneWeapon<'source> {
    pub(crate) fn as_memo(&'source self) -> OneHandedMundaneWeaponMemo {
        OneHandedMundaneWeaponMemo(self.0.clone())
    }

    pub(crate) fn name(&self) -> &'source str {
        self.0.name.as_str()
    }
}