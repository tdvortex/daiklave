use std::num::NonZeroU8;

use crate::CharacterMutation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GainWillpower(pub NonZeroU8);

impl From<GainWillpower> for CharacterMutation {
    fn from(gain_willpower: GainWillpower) -> Self {
        CharacterMutation::GainWillpower(gain_willpower)
    }
}