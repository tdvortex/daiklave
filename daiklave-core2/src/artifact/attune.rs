use crate::{CharacterMutation, exaltation::exalt::essence::MotePoolName};

use super::ArtifactNameMutation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttuneArtifact{
    pub artifact_name: ArtifactNameMutation,
    pub first: MotePoolName,
}

impl From<AttuneArtifact> for CharacterMutation {
    fn from(attune_artifact: AttuneArtifact) -> Self {
        CharacterMutation::AttuneArtifact(attune_artifact)
    }
}