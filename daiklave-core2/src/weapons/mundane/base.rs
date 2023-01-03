use std::ops::Deref;

use serde::{Serialize, Deserialize};

use crate::weapons::base::{BaseWeapon, BaseWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons::mundane) struct BaseMundaneWeapon<'source>(BaseWeapon<'source>);

impl<'source> Deref for BaseMundaneWeapon<'source> {
    type Target = BaseWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons::mundane) struct BaseMundaneWeaponMemo(BaseWeaponMemo);

impl<'source> BaseMundaneWeaponMemo {
    pub fn as_ref(&'source self) -> BaseMundaneWeapon<'source> {
        BaseMundaneWeapon(self.0.as_ref())
    }
}