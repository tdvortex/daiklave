use std::num::NonZeroU8;

use crate::{CharacterMutation, exaltation::exalt::essence::MotePoolName};

pub struct SpendMotes {
    first: MotePoolName,
    amount: NonZeroU8,
}

impl SpendMotes {
    pub fn new(first: MotePoolName, amount: NonZeroU8) -> Self {
        Self {
            first,
            amount,
        }
    }
}

impl From<SpendMotes> for CharacterMutation {
    fn from(spend_motes: SpendMotes) -> Self {
        CharacterMutation::SpendMotes(spend_motes)
    }
}