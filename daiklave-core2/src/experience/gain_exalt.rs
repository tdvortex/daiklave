use std::num::NonZeroU16;

use crate::CharacterMutation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GainExaltExperience(pub NonZeroU16);

impl From<GainExaltExperience> for CharacterMutation {
    fn from(gain_exalt_experience: GainExaltExperience) -> Self {
        CharacterMutation::GainExaltExperience(gain_exalt_experience)
    }
}