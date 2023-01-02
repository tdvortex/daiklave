use std::ops::Deref;

use crate::weapons::base::BaseWeapon;

pub(in crate::weapons::mundane) struct BaseMundaneWeapon<'source>(BaseWeapon<'source>);

impl<'source> Deref for BaseMundaneWeapon<'source> {
    type Target = BaseWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}