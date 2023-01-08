mod memo;

use std::ops::Deref;

pub use memo::OneHandedMundaneWeapon;

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

impl<'source> OneHandedMundaneWeaponView<'source> {
    pub(crate) fn as_memo(&'source self) -> OneHandedMundaneWeapon {
        OneHandedMundaneWeapon(self.0.clone())
    }

    pub(crate) fn name(&self) -> &'source str {
        self.0.name.as_str()
    }
}
