use crate::{exaltation::exalt::essence::mote_commitment::MoteCommitmentNameMutation, CharacterMutation};

/// An instruction to uncommit a specific mote commitment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UncommitMotes(pub MoteCommitmentNameMutation);

impl From<UncommitMotes> for CharacterMutation {
    fn from(uncommit_motes: UncommitMotes) -> Self {
        CharacterMutation::UncommitMotes(uncommit_motes)
    }
}