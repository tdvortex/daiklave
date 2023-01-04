use std::ops::Deref;

use serde::{Deserialize, Serialize};

use super::named::{NamedArtifactWeapon, NamedArtifactWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WornArtifactWeapon<'source>(NamedArtifactWeapon<'source>);

impl<'source> Deref for WornArtifactWeapon<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> WornArtifactWeapon<'source> {
    pub fn as_memo(&'source self) -> WornArtifactWeaponMemo {
        WornArtifactWeaponMemo(self.0.as_memo())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct WornArtifactWeaponMemo(NamedArtifactWeaponMemo);

impl<'source> WornArtifactWeaponMemo {
    pub fn as_ref(&'source self) -> WornArtifactWeapon<'source> {
        WornArtifactWeapon(self.0.as_ref())
    }
}
