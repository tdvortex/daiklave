use crate::{CharacterMutation, exaltation::exalt::essence::MotePoolName};

use super::{ArtifactNameMutation, ArtifactName};

/// A command to attune to a specific artifact. Requires specifying both the
/// artifact to attune to, and whether the mote commitment should draw from
/// Personal or Peripheral motes first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttuneArtifact{
    pub(crate) artifact_name: ArtifactNameMutation,
    pub(crate) first: MotePoolName,
}

impl AttuneArtifact {
    /// Creates a new AttuneArtifact mutation.
    pub fn new(artifact_name: ArtifactName<'_>, first: MotePoolName) -> Self {
        Self {
            artifact_name: artifact_name.into(),
            first,
        }
    }
}

impl From<AttuneArtifact> for CharacterMutation {
    fn from(attune_artifact: AttuneArtifact) -> Self {
        CharacterMutation::AttuneArtifact(attune_artifact)
    }
}