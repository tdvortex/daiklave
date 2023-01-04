use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::weapons::base::BaseWeapon;

use super::base::{BaseMundaneWeapon, BaseMundaneWeaponMemo};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TwoHandedMundaneWeapon<'source>(BaseMundaneWeapon<'source>);

impl<'source> Deref for TwoHandedMundaneWeapon<'source> {
    type Target = BaseWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<'source> TwoHandedMundaneWeapon<'source> {
    pub fn as_memo(&'source self) -> TwoHandedMundaneWeaponMemo {
        TwoHandedMundaneWeaponMemo(self.0.as_memo())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TwoHandedMundaneWeaponMemo(BaseMundaneWeaponMemo);

impl<'source> TwoHandedMundaneWeaponMemo {
    pub fn as_ref(&'source self) -> TwoHandedMundaneWeapon<'source> {
        TwoHandedMundaneWeapon(self.0.as_ref())
    }
}
