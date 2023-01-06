use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::weapons::base::{BaseWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WornMundaneWeapon<'source>(pub(crate) &'source BaseWeaponMemo);

impl<'source> Deref for WornMundaneWeapon<'source> {
    type Target = BaseWeaponMemo;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'source> WornMundaneWeapon<'source> {
    pub fn as_memo(&'source self) -> WornMundaneWeaponMemo {
        WornMundaneWeaponMemo(self.0.clone())
    }

    pub fn name(&self) -> &'source str {
        self.0.name.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WornMundaneWeaponMemo(pub(crate) BaseWeaponMemo);

impl<'source> WornMundaneWeaponMemo {
    pub fn as_ref(&'source self) -> WornMundaneWeapon<'source> {
        WornMundaneWeapon(&self.0)
    }
}
