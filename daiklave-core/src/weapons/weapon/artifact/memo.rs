use serde::{Serialize, Deserialize};

use crate::{
    artifact::AddArtifact,
    weapons::weapon::builder::base::{BaseArtifactWeaponBuilder, BaseWeaponBuilder},
    CharacterMutation,
};

use super::{
    builder::{ArtifactWeaponBuilder, ArtifactWeaponBuilderWithHearthstoneSlots},
    handedness::ArtifactWeaponHandedness,
    AddBaseArtifactWeapon, ArtifactWeaponName,
};

/// A mutation to add an artifact weapon to a character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddArtifactWeapon {
    pub(crate) name: ArtifactWeaponName,
    pub(crate) handedness: ArtifactWeaponHandedness,
}

impl AddArtifactWeapon {
    /// Starts constructing a new base artifact weapon as part of the builder
    /// process.
    pub fn new_base_weapon(name: impl Into<String>) -> BaseArtifactWeaponBuilder {
        BaseWeaponBuilder::name(name).artifact()
    }

    /// Uses an existing weapon as a base to start creating a new artifact weapon.
    pub fn with_base_weapon(
        add_base_weapon: impl Into<AddBaseArtifactWeapon>,
    ) -> ArtifactWeaponBuilder {
        ArtifactWeaponBuilder::base_weapon(add_base_weapon)
    }
}

impl From<ArtifactWeaponBuilderWithHearthstoneSlots> for AddArtifactWeapon {
    fn from(builder: ArtifactWeaponBuilderWithHearthstoneSlots) -> Self {
        builder.build()
    }
}

impl From<AddArtifactWeapon> for CharacterMutation {
    fn from(add_artifact_weapon: AddArtifactWeapon) -> Self {
        AddArtifact::from(add_artifact_weapon).into()
    }
}
