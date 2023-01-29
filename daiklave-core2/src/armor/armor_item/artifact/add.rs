use crate::{CharacterMutation, artifact::AddArtifact};

use super::{ArtifactArmor, ArtifactArmorName};

/// The name and details of a unique piece of artifact armor to be added to a
/// character.
pub struct AddArtifactArmor {
    pub(crate) name: ArtifactArmorName,
    pub(crate) armor: ArtifactArmor,
}

impl From<AddArtifactArmor> for CharacterMutation {
    fn from(add_artifact_armor: AddArtifactArmor) -> Self {
        let add_artifact: AddArtifact = add_artifact_armor.into();
        add_artifact.into()
    }
}