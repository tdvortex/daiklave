use std::num::NonZeroU8;

use crate::CharacterMutation;

pub struct GainWillpower(NonZeroU8);

impl GainWillpower {
    pub fn new(amount: NonZeroU8) -> Self {
        Self(amount)
    }
}

impl From<GainWillpower> for CharacterMutation {
    fn from(gain_willpower: GainWillpower) -> Self {
        CharacterMutation::GainWillpower(gain_willpower)
    }
}