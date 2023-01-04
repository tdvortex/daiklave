use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::weapons::base::BaseWeapon;

use super::base::{BaseMundaneWeapon, BaseMundaneWeaponMemo};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NaturalMundaneWeapon<'source>(BaseMundaneWeapon<'source>);

impl<'source> Deref for NaturalMundaneWeapon<'source> {
    type Target = BaseWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<'source> NaturalMundaneWeapon<'source> {
    pub fn as_memo(&'source self) -> NaturalMundaneWeaponMemo {
        NaturalMundaneWeaponMemo(self.0.as_memo())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NaturalMundaneWeaponMemo(BaseMundaneWeaponMemo);

impl<'source> NaturalMundaneWeaponMemo {
    pub fn as_ref(&'source self) -> NaturalMundaneWeapon<'source> {
        NaturalMundaneWeapon(self.0.as_ref())
    }
}
