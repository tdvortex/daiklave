use std::num::NonZeroU8;

use crate::{CharacterMutation, exaltation::exalt::essence::MotePoolName};

/// A mutation to spend some number of motes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpendMotes {
    /// Indicates whether to spend the motes out of peripheral or personal
    /// first. Will overflow to the other pool if necessary.
    pub first: MotePoolName,
    /// The number of motes to spend.
    pub amount: NonZeroU8,
}

impl From<SpendMotes> for CharacterMutation {
    fn from(spend_motes: SpendMotes) -> Self {
        CharacterMutation::SpendMotes(spend_motes)
    }
}