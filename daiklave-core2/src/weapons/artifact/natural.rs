use std::ops::Deref;

use serde::{Serialize, Deserialize};

use super::named::{NamedArtifactWeapon, NamedArtifactWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NaturalArtifactWeapon<'source>(NamedArtifactWeapon<'source>);

impl<'source> Deref for NaturalArtifactWeapon<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> NaturalArtifactWeapon<'source> {
    pub fn as_memo(&'source self) -> NaturalArtifactWeaponMemo {
        NaturalArtifactWeaponMemo(self.0.as_memo())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NaturalArtifactWeaponMemo(NamedArtifactWeaponMemo);

impl<'source> NaturalArtifactWeaponMemo {
    pub fn as_ref(&'source self) -> NaturalArtifactWeapon<'source> {
        NaturalArtifactWeapon(self.0.as_ref())
    }
}