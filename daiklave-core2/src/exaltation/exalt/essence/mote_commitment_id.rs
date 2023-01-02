use serde::{Deserialize, Serialize};

use crate::{unique_id::UniqueId, weapons::ArtifactId};

/// A unique identifier for a mote commitment effect.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoteCommitmentId {
    AttunedArtifact(ArtifactId),
    Other(UniqueId),
}