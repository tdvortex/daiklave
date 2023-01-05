use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::weapons::base::BaseWeapon;

use super::base::{BaseMundaneWeapon, BaseMundaneWeaponMemo};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WornMundaneWeapon<'source>(BaseMundaneWeapon<'source>);

impl<'source> Deref for WornMundaneWeapon<'source> {
    type Target = BaseWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> WornMundaneWeapon<'source> {
    pub fn as_memo(&'source self) -> WornMundaneWeaponMemo {
        WornMundaneWeaponMemo(self.0.as_memo())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WornMundaneWeaponMemo(BaseMundaneWeaponMemo);

impl<'source> WornMundaneWeaponMemo {
    pub fn as_ref(&'source self) -> WornMundaneWeapon<'source> {
        WornMundaneWeapon(self.0.as_ref())
    }
}
