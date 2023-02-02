use crate::{CharacterMutation, merits::merit::RemoveMerit};

use super::{ArtifactNameMutation, ArtifactName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveArtifact(pub(crate) ArtifactNameMutation);

impl RemoveArtifact {
    pub fn new(name: ArtifactName<'_>) -> Self {
        Self(name.into())
    }
}

impl From<RemoveArtifact> for CharacterMutation {
    fn from(remove_artifact: RemoveArtifact) -> Self {
        RemoveMerit::from(remove_artifact).into()
    }
}