use serde::{Serialize, Deserialize};

use crate::{merits::merit::RemoveMerit, CharacterMutation};

use super::{ArtifactName, ArtifactNameMutation};

/// A mutation to remove an artifact from a character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoveArtifact(pub(crate) ArtifactNameMutation);

impl RemoveArtifact {
    /// Remove the artifact with this name.
    pub fn new(name: ArtifactName<'_>) -> Self {
        Self(name.into())
    }
}

impl From<RemoveArtifact> for CharacterMutation {
    fn from(remove_artifact: RemoveArtifact) -> Self {
        RemoveMerit::from(remove_artifact).into()
    }
}
