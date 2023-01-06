use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::weapons::base::{BaseWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneHandedMundaneWeapon<'source>(pub(crate) &'source BaseWeaponMemo);

impl<'source> Deref for OneHandedMundaneWeapon<'source> {
    type Target = BaseWeaponMemo;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'source> OneHandedMundaneWeapon<'source> {
    pub fn as_memo(&'source self) -> OneHandedMundaneWeaponMemo {
        OneHandedMundaneWeaponMemo(self.0.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OneHandedMundaneWeaponMemo(pub(crate) BaseWeaponMemo);

impl<'source> OneHandedMundaneWeaponMemo {
    pub fn as_ref(&'source self) -> OneHandedMundaneWeapon<'source> {
        OneHandedMundaneWeapon(&self.0)
    }
}
