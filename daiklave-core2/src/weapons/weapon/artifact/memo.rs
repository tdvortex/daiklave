use crate::{CharacterMutation, artifact::AddArtifact};

use super::{ArtifactWeaponName, handedness::ArtifactWeaponHandedness, builder::ArtifactWeaponBuilderWithHearthstoneSlots};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddArtifactWeapon {
    pub(crate) name: ArtifactWeaponName,
    pub(crate) handedness: ArtifactWeaponHandedness
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