mod other;
pub use other::OtherMoteCommitmentId;
use serde::{Deserialize, Serialize};

use crate::artifact::ArtifactId;

/// A unique identifier for a mote commitment effect.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoteCommitmentId {
    /// Attuning to an artifact requires a mote commitment
    AttunedArtifact(ArtifactId),
    /// Other effects may also require mote commitments
    Other(OtherMoteCommitmentId),
}
