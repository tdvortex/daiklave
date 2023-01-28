use serde::{Deserialize, Serialize};

use crate::artifact::ArtifactNameMutation;

/// An instruction to uncommit a specific mote commitment.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum UncommitMotes {
    /// Unattune from this artifact.
    UnattuneArtifact(ArtifactNameMutation),
    /// Uncommit some other mote effect.
    Other(String),
}
