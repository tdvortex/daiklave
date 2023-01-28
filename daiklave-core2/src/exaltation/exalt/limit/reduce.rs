use std::num::NonZeroU8;

use crate::CharacterMutation;

pub struct ReduceLimit(NonZeroU8);

impl ReduceLimit {
    pub fn new(amount: NonZeroU8) -> Self {
        Self(amount)
    }
}

impl From<ReduceLimit> for CharacterMutation {
    fn from(reduce_limit: ReduceLimit) -> Self {
        Self::ReduceLimit(reduce_limit)
    }
}