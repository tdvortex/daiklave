use serde::{Deserialize, Serialize};

use crate::artifact::ArtifactId;

/// A unique identifier for a mote commitment effect.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoteCommitmentId<'source> {
    /// Attuning to an artifact requires a mote commitment
    AttunedArtifact(ArtifactId<'source>),
    /// Other effects may also require mote commitments
    Other(&'source str),
}
