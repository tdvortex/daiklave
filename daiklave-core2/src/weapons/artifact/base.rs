use std::ops::Deref;

use serde::{Serialize, Deserialize};

use crate::weapons::base::{BaseWeapon, BaseWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseArtifactWeapon<'source>(BaseWeapon<'source>);

impl<'source> Deref for BaseArtifactWeapon<'source> {
    type Target = BaseWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaseArtifactWeaponMemo(BaseWeaponMemo);

impl<'source> BaseArtifactWeaponMemo {
    pub fn as_ref(&'source self) -> BaseArtifactWeapon<'source> {
        BaseArtifactWeapon(self.0.as_ref())
    }
}