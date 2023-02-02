use std::num::NonZeroU8;

use crate::CharacterMutation;

/// A mutation to increase the Limit track of a Celestial Exalted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GainLimit(pub NonZeroU8);

impl From<GainLimit> for CharacterMutation {
    fn from(gain_limit: GainLimit) -> Self {
        Self::GainLimit(gain_limit)
    }
}
