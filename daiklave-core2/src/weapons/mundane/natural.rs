use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::weapons::base::{BaseWeapon, BaseWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NaturalMundaneWeapon<'source>(pub(crate) BaseWeapon<'source>);

impl<'source> Deref for NaturalMundaneWeapon<'source> {
    type Target = BaseWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> NaturalMundaneWeapon<'source> {
    pub fn as_memo(&'source self) -> NaturalMundaneWeaponMemo {
        NaturalMundaneWeaponMemo(self.0.as_memo())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NaturalMundaneWeaponMemo(BaseWeaponMemo);

impl<'source> NaturalMundaneWeaponMemo {
    pub fn as_ref(&'source self) -> NaturalMundaneWeapon<'source> {
        NaturalMundaneWeapon(self.0.as_ref())
    }
}
