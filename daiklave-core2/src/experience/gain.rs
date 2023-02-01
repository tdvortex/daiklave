use std::num::NonZeroU16;

use crate::CharacterMutation;

/// A mutation to increase the amount of experience for a character. This 
/// increases both current and total.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GainExperience(pub NonZeroU16);

impl From<GainExperience> for CharacterMutation {
    fn from(gain_experience: GainExperience) -> Self {
        CharacterMutation::GainExperience(gain_experience)
    }
}