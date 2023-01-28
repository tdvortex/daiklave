use serde::{Serialize, Deserialize};

use crate::{exaltation::exalt::essence::mote_commitment::MoteCommitmentNameMutation, CharacterMutation};

/// An instruction to uncommit a specific mote commitment.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UncommitMotes(MoteCommitmentNameMutation);

impl From<MoteCommitmentNameMutation> for UncommitMotes {
    fn from(name: MoteCommitmentNameMutation) -> Self {
        Self(name)
    }
}

impl From<UncommitMotes> for CharacterMutation {
    fn from(uncommit_motes: UncommitMotes) -> Self {
        CharacterMutation::UncommitMotes(uncommit_motes)
    }
}