mod memo;

use std::ops::Deref;

use crate::weapons::weapon::base::BaseWeapon;

pub use self::memo::WornMundaneWeaponMemo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WornMundaneWeapon<'source>(pub(crate) &'source BaseWeapon);

impl<'source> Deref for WornMundaneWeapon<'source> {
    type Target = BaseWeapon;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'source> WornMundaneWeapon<'source> {
    pub fn as_memo(&'source self) -> WornMundaneWeaponMemo {
        WornMundaneWeaponMemo(self.0.clone())
    }

    pub fn name(&self) -> &'source str {
        self.0.name.as_str()
    }
}
