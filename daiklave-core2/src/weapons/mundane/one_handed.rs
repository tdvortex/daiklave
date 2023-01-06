use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::weapons::base::{BaseWeapon, BaseWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneHandedMundaneWeapon<'source>(pub(crate) BaseWeapon<'source>);

impl<'source> Deref for OneHandedMundaneWeapon<'source> {
    type Target = BaseWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> OneHandedMundaneWeapon<'source> {
    pub fn as_memo(&'source self) -> OneHandedMundaneWeaponMemo {
        OneHandedMundaneWeaponMemo(self.0.as_memo())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OneHandedMundaneWeaponMemo(BaseWeaponMemo);

impl<'source> OneHandedMundaneWeaponMemo {
    pub fn as_ref(&'source self) -> OneHandedMundaneWeapon<'source> {
        OneHandedMundaneWeapon(self.0.as_ref())
    }
}
