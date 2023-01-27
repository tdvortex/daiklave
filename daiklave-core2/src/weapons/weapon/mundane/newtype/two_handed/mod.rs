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

impl<'source> TwoHandedMundaneWeaponView<'source> {
    pub fn as_memo(&'source self) -> TwoHandedMundaneWeapon {
        TwoHandedMundaneWeapon(self.0.clone())
    }
}
