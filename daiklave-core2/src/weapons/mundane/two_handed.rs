use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::weapons::base::{BaseWeapon, BaseWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwoHandedMundaneWeapon<'source>(pub(crate) BaseWeapon<'source>);

impl<'source> Deref for TwoHandedMundaneWeapon<'source> {
    type Target = BaseWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> TwoHandedMundaneWeapon<'source> {
    pub fn as_memo(&'source self) -> TwoHandedMundaneWeaponMemo {
        TwoHandedMundaneWeaponMemo(self.0.as_memo())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwoHandedMundaneWeaponMemo(BaseWeaponMemo);

impl<'source> TwoHandedMundaneWeaponMemo {
    pub fn as_ref(&'source self) -> TwoHandedMundaneWeapon<'source> {
        TwoHandedMundaneWeapon(self.0.as_ref())
    }
}
