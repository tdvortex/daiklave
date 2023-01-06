use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::weapons::base::{BaseWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NaturalMundaneWeapon<'source>(pub(crate) &'source BaseWeaponMemo);

impl<'source> Deref for NaturalMundaneWeapon<'source> {
    type Target = BaseWeaponMemo;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'source> NaturalMundaneWeapon<'source> {
    pub fn as_memo(&'source self) -> NaturalMundaneWeaponMemo {
        NaturalMundaneWeaponMemo(self.0.clone())
    }

    pub fn name(&self) -> &'source str {
        self.0.name.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NaturalMundaneWeaponMemo(pub(crate) BaseWeaponMemo);

impl<'source> NaturalMundaneWeaponMemo {
    pub fn as_ref(&'source self) -> NaturalMundaneWeapon<'source> {
        NaturalMundaneWeapon(&self.0)
    }
}
