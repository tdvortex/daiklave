mod memo;
mod unarmed;

pub use unarmed::unarmed;

use std::ops::Deref;

use crate::weapons::weapon::base::BaseWeapon;

pub use self::memo::NaturalMundaneWeapon;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NaturalMundaneWeaponView<'source>(pub(crate) &'source BaseWeapon);

impl<'source> Deref for NaturalMundaneWeaponView<'source> {
    type Target = BaseWeapon;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'source> NaturalMundaneWeaponView<'source> {
    pub fn as_memo(&'source self) -> NaturalMundaneWeapon {
        NaturalMundaneWeapon(self.0.clone())
    }

    pub fn name(&self) -> &'source str {
        self.0.name.as_str()
    }
}
