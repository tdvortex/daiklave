use std::ops::Deref;

use serde::{Serialize, Deserialize};

use crate::weapons::base::BaseWeapon;

use super::base::{BaseMundaneWeapon, BaseMundaneWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneHandedMundaneWeapon<'source>(BaseMundaneWeapon<'source>);

impl<'source> Deref for OneHandedMundaneWeapon<'source> {
    type Target = BaseWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OneHandedMundaneWeaponMemo(BaseMundaneWeaponMemo);

impl<'source> OneHandedMundaneWeaponMemo {
    pub fn as_ref(&'source self) -> OneHandedMundaneWeapon<'source> {
        OneHandedMundaneWeapon(self.0.as_ref())
    }
}