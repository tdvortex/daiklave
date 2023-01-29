use crate::CharacterMutation;

use super::{artifact::AddArtifactArmor, mundane::AddMundaneArmor};

pub enum AddArmor {
    Artifact(AddArtifactArmor),
    Mundane(AddMundaneArmor),
}

impl From<AddArtifactArmor> for AddArmor {
    fn from(add_artifact: AddArtifactArmor) -> Self {
        Self::Artifact(add_artifact)
    }
}

impl From<AddMundaneArmor> for AddArmor {
    fn from(add_mundane: AddMundaneArmor) -> Self {
        Self::Mundane(add_mundane)
    }
}

impl From<AddArmor> for CharacterMutation {
    fn from(add_armor: AddArmor) -> Self {
        match add_armor {
            AddArmor::Artifact(add_artifact) => add_artifact.into(),
            AddArmor::Mundane(add_mundane) => add_mundane.into(),
        }
    }
}