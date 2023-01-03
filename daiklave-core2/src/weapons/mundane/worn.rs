use std::ops::Deref;

use serde::{Serialize, Deserialize};

use crate::weapons::base::BaseWeapon;

use super::base::{BaseMundaneWeapon, BaseMundaneWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WornMundaneWeapon<'source>(BaseMundaneWeapon<'source>);

impl<'source> Deref for WornMundaneWeapon<'source> {
    type Target = BaseWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WornMundaneWeaponMemo(BaseMundaneWeaponMemo);

impl<'source> WornMundaneWeaponMemo {
    pub fn as_ref(&'source self) -> WornMundaneWeapon<'source> {
        WornMundaneWeapon(self.0.as_ref())
    }
}