mod memo;
mod unarmed;

pub use unarmed::unarmed;

use std::ops::Deref;

use crate::weapons::weapon::base::BaseWeaponMemo;

pub use self::memo::NaturalMundaneWeaponMemo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NaturalMundaneWeapon<'source>(pub(crate) &'source BaseWeaponMemo);

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
