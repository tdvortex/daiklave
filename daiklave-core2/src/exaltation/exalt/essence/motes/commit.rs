use std::num::NonZeroU8;

use crate::{exaltation::exalt::essence::{mote_commitment::OtherMoteCommitmentName, MotePoolName}, CharacterMutation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitMotes {
    pub effect_name: OtherMoteCommitmentName,
    pub first: MotePoolName,
    pub amount: NonZeroU8,
}


impl From<CommitMotes> for CharacterMutation {
    fn from(commit_motes: CommitMotes) -> Self {
        Self::CommitMotes(commit_motes)
    }
}