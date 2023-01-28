use crate::{CharacterMutation, exaltation::exalt::essence::MotePoolName};

use super::ArtifactNameMutation;

pub struct AttuneArtifact{
    artifact_name: ArtifactNameMutation,
    first: MotePoolName,
}

impl AttuneArtifact {
    pub fn new(artifact_name: ArtifactNameMutation, first: MotePoolName) -> Self {
        Self {
            artifact_name,
            first,
        }
    }
}

impl From<AttuneArtifact> for CharacterMutation {
    fn from(attune_artifact: AttuneArtifact) -> Self {
        CharacterMutation::AttuneArtifact(attune_artifact)
    }
}