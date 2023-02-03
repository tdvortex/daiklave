use std::ops::Deref;

use serde::{Deserialize, Serialize};

use super::{
    builder::{ArtifactWeaponBuilder, ArtifactWeaponBuilderWithName},
    AddBaseArtifactWeapon,
};

/// The name of an artifact weapon.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct ArtifactWeaponName(String);

impl ArtifactWeaponName {
    /// Constructs a weapon with this name using this base artifact weapon.
    pub fn with_base_weapon(
        self,
        add_base_weapon: impl Into<AddBaseArtifactWeapon>,
    ) -> ArtifactWeaponBuilderWithName {
        ArtifactWeaponBuilder::base_weapon(add_base_weapon).name(self)
    }
}

impl<T> From<T> for ArtifactWeaponName
where
    T: Into<String>,
{
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for ArtifactWeaponName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
