use std::num::NonZeroU8;

use crate::{CharacterMutation, exaltation::exalt::essence::MotePoolName};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpendMotes {
    pub first: MotePoolName,
    pub amount: NonZeroU8,
}

impl From<SpendMotes> for CharacterMutation {
    fn from(spend_motes: SpendMotes) -> Self {
        CharacterMutation::SpendMotes(spend_motes)
    }
}