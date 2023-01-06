mod memo;

use std::ops::Deref;

use crate::weapons::weapon::base::BaseWeapon;

pub use self::memo::TwoHandedMundaneWeaponMemo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwoHandedMundaneWeapon<'source>(pub(crate) &'source BaseWeapon);

impl<'source> Deref for TwoHandedMundaneWeapon<'source> {
    type Target = BaseWeapon;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'source> TwoHandedMundaneWeapon<'source> {
    pub fn as_memo(&'source self) -> TwoHandedMundaneWeaponMemo {
        TwoHandedMundaneWeaponMemo(self.0.clone())
    }

    pub fn name(&self) -> &'source str {
        self.0.name.as_str()
    }
}
