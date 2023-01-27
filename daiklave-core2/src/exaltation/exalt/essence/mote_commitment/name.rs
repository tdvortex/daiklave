use serde::{Deserialize, Serialize};

use crate::artifact::ArtifactName;

/// An instruction to uncommit a specific mote commitment.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum UncommitMotes {
    /// Unattune from this artifact.
    UnattuneArtifact(ArtifactName),
    /// Uncommit some other mote effect.
    Other(String),
}
