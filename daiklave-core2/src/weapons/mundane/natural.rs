use std::ops::Deref;

use crate::weapons::base::BaseWeapon;

use super::base::BaseMundaneWeapon;

pub struct NaturalMundaneWeapon<'source>(BaseMundaneWeapon<'source>);

impl<'source> Deref for NaturalMundaneWeapon<'source> {
    type Target = BaseWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}