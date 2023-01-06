use std::ops::Deref;

use serde::{Deserialize, Serialize};

use super::named::{NamedArtifactWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WornArtifactWeapon<'source>(pub(crate) &'source NamedArtifactWeaponMemo);

impl<'source> Deref for WornArtifactWeapon<'source> {
    type Target = NamedArtifactWeaponMemo;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'source> WornArtifactWeapon<'source> {
    pub fn as_memo(&'source self) -> WornArtifactWeaponMemo {
        WornArtifactWeaponMemo(self.0.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WornArtifactWeaponMemo(pub(crate) NamedArtifactWeaponMemo);

impl<'source> WornArtifactWeaponMemo {
    pub fn as_ref(&'source self) -> WornArtifactWeapon<'source> {
        WornArtifactWeapon(&self.0)
    }
}
