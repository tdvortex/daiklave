mod mutation;
mod other;
pub(crate) use mutation::MoteCommitmentNameMutation;
pub(crate) use other::OtherMoteCommitmentName;

use crate::artifact::ArtifactName;

/// The name of a mote commitment effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoteCommitmentName<'source> {
    /// The character has attuned to an artifact.
    AttunedArtifact(ArtifactName<'source>),
    /// The character has committed motes to some other effect, like a Charm.
    Other(&'source str),
}