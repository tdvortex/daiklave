use std::ops::Deref;

use serde::{Serialize, Deserialize};

use super::named::{NamedArtifactWeapon, NamedArtifactWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwoHandedArtifactWeapon<'source>(NamedArtifactWeapon<'source>);

impl<'source> Deref for TwoHandedArtifactWeapon<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> TwoHandedArtifactWeapon<'source> {
    pub fn as_memo(&'source self) -> TwoHandedArtifactWeaponMemo {
        TwoHandedArtifactWeaponMemo(self.0.as_memo())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwoHandedArtifactWeaponMemo(NamedArtifactWeaponMemo);

impl<'source> TwoHandedArtifactWeaponMemo {
    pub fn as_ref(&'source self) -> TwoHandedArtifactWeapon<'source> {
        TwoHandedArtifactWeapon(self.0.as_ref())
    }
}