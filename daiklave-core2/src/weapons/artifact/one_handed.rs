use std::ops::Deref;

use serde::{Deserialize, Serialize};

use super::named::{NamedArtifactWeapon, NamedArtifactWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneHandedArtifactWeapon<'source>(NamedArtifactWeapon<'source>);

impl<'source> Deref for OneHandedArtifactWeapon<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> OneHandedArtifactWeapon<'source> {
    pub fn as_memo(&'source self) -> OneHandedArtifactWeaponMemo {
        OneHandedArtifactWeaponMemo(self.0.as_memo())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OneHandedArtifactWeaponMemo(NamedArtifactWeaponMemo);

impl<'source> OneHandedArtifactWeaponMemo {
    pub fn as_ref(&'source self) -> OneHandedArtifactWeapon<'source> {
        OneHandedArtifactWeapon(self.0.as_ref())
    }
}
