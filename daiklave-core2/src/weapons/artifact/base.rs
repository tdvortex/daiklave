use std::ops::Deref;

use crate::weapons::base::BaseWeapon;

pub struct BaseArtifactWeapon<'source>(BaseWeapon<'source>);

impl<'source> Deref for BaseArtifactWeapon<'source> {
    type Target = BaseWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}