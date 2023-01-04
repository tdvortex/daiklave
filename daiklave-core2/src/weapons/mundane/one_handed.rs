use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::weapons::base::BaseWeapon;

use super::base::{BaseMundaneWeapon, BaseMundaneWeaponMemo};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OneHandedMundaneWeapon<'source>(BaseMundaneWeapon<'source>);

impl<'source> Deref for OneHandedMundaneWeapon<'source> {
    type Target = BaseWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<'source> OneHandedMundaneWeapon<'source> {
    pub fn as_memo(&'source self) -> OneHandedMundaneWeaponMemo {
        OneHandedMundaneWeaponMemo(self.0.as_memo())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct OneHandedMundaneWeaponMemo(BaseMundaneWeaponMemo);

impl<'source> OneHandedMundaneWeaponMemo {
    pub fn as_ref(&'source self) -> OneHandedMundaneWeapon<'source> {
        OneHandedMundaneWeapon(self.0.as_ref())
    }
}
