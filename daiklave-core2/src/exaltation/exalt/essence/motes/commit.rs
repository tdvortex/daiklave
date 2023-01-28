use std::num::NonZeroU8;

use crate::exaltation::exalt::essence::{mote_commitment::OtherMoteCommitmentName, MotePoolName};

pub struct CommitMotes {
    effect_name: OtherMoteCommitmentName,
    first: MotePoolName,
    amount: NonZeroU8,
}

impl CommitMotes {
    pub fn new(effect_name: String, first: MotePoolName, amount: NonZeroU8) -> Self {
        Self {
            effect_name: effect_name.into(),
            first,
            amount,
        }
    }
}