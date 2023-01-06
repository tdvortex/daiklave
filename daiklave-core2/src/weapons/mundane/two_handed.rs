use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::weapons::base::{BaseWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwoHandedMundaneWeapon<'source>(pub(crate) &'source BaseWeaponMemo);

impl<'source> Deref for TwoHandedMundaneWeapon<'source> {
    type Target = BaseWeaponMemo;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'source> TwoHandedMundaneWeapon<'source> {
    pub fn as_memo(&'source self) -> TwoHandedMundaneWeaponMemo {
        TwoHandedMundaneWeaponMemo(self.0.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwoHandedMundaneWeaponMemo(pub(crate) BaseWeaponMemo);

impl<'source> TwoHandedMundaneWeaponMemo {
    pub fn as_ref(&'source self) -> TwoHandedMundaneWeapon<'source> {
        TwoHandedMundaneWeapon(&self.0)
    }
}
