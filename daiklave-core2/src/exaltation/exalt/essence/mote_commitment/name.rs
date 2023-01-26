use serde::{Serialize, Deserialize};

use crate::artifact::ArtifactId;

/// An instruction to uncommit a specific mote commitment.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum UncommitMotes {
    /// Unattune from this artifact.
    UnattuneArtifact(ArtifactId),
    /// Uncommit some other mote effect.
    Other(String),
}