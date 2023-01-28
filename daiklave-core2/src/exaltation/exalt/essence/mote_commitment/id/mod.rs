use crate::artifact::ArtifactName;

/// A unique identifier for a mote commitment effect.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MoteCommitmentId<'source> {
    /// Attuning to an artifact requires a mote commitment
    AttunedArtifact(ArtifactName<'source>),
    /// Other effects may also require mote commitments
    Other(&'source str),
}
