use std::ops::Deref;

use serde::{Serialize, Deserialize};

use super::named::{NamedArtifactWeapon, NamedArtifactWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WornArtifactWeapon<'source>(NamedArtifactWeapon<'source>);

impl<'source> Deref for WornArtifactWeapon<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WornArtifactWeaponMemo(NamedArtifactWeaponMemo);

impl<'source> WornArtifactWeaponMemo {
    pub fn as_ref(&'source self) -> WornArtifactWeapon<'source> {
        WornArtifactWeapon(self.0.as_ref())
    }
}