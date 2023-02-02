use std::num::NonZeroU8;

use crate::CharacterMutation;

/// A mutation to reduce the Limit of a Celestial Exalted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReduceLimit(pub NonZeroU8);

impl From<ReduceLimit> for CharacterMutation {
    fn from(reduce_limit: ReduceLimit) -> Self {
        Self::ReduceLimit(reduce_limit)
    }
}
