use std::num::NonZeroU8;

use crate::{
    exaltation::exalt::essence::{mote_commitment::OtherMoteCommitmentName, MotePoolName},
    CharacterMutation,
};

/// A mutation to commit motes to an effect other than attuning to
/// and artifact. For artifact attunements, use [AttuneArtifact].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitMotes {
    /// The name of the effect.
    pub effect_name: OtherMoteCommitmentName,
    /// Indicates whether to commit motes out of the peripheral or personal
    /// pool first.
    pub first: MotePoolName,
    /// The quantity of motes to commit.
    pub amount: NonZeroU8,
}

impl From<CommitMotes> for CharacterMutation {
    fn from(commit_motes: CommitMotes) -> Self {
        Self::CommitMotes(commit_motes)
    }
}
