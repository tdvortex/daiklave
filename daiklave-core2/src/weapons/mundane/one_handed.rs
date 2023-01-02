use std::ops::Deref;

use crate::weapons::base::BaseWeapon;

use super::base::BaseMundaneWeapon;

pub struct OneHandedMundaneWeapon<'source>(BaseMundaneWeapon<'source>);

impl<'source> Deref for OneHandedMundaneWeapon<'source> {
    type Target = BaseWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}