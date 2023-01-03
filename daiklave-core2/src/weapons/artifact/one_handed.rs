use std::ops::Deref;

use serde::{Serialize, Deserialize};

use super::named::{NamedArtifactWeapon, NamedArtifactWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneHandedArtifactWeapon<'source>(NamedArtifactWeapon<'source>);

impl<'source> Deref for OneHandedArtifactWeapon<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OneHandedArtifactWeaponMemo(NamedArtifactWeaponMemo);

impl<'source> OneHandedArtifactWeaponMemo {
    pub fn as_ref(&'source self) -> OneHandedArtifactWeapon<'source> {
        OneHandedArtifactWeapon(self.0.as_ref())
    }
}