use std::num::NonZeroU8;

use crate::CharacterMutation;

pub struct GainLimit(NonZeroU8);

impl GainLimit {
    pub fn new(amount: NonZeroU8) -> Self {
        Self(amount)
    }
}

impl From<GainLimit> for CharacterMutation {
    fn from(gain_limit: GainLimit) -> Self {
        Self::GainLimit(gain_limit)
    }
}