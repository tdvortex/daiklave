use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::weapons::base::{BaseWeapon, BaseWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BaseArtifactWeapon<'source>(BaseWeapon<'source>);

impl<'source> Deref for BaseArtifactWeapon<'source> {
    type Target = BaseWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> BaseArtifactWeapon<'source> {
    pub fn as_memo(&self) -> BaseArtifactWeaponMemo {
        BaseArtifactWeaponMemo(self.0.as_memo())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons) struct BaseArtifactWeaponMemo(BaseWeaponMemo);

impl<'source> BaseArtifactWeaponMemo {
    pub fn as_ref(&'source self) -> BaseArtifactWeapon<'source> {
        BaseArtifactWeapon(self.0.as_ref())
    }
}
