use std::num::NonZeroU16;

use crate::CharacterMutation;

pub struct GainExaltExperience(NonZeroU16);

impl GainExaltExperience {
    pub fn new(amount: NonZeroU16) -> Self {
        Self(amount)
    }
}

impl From<GainExaltExperience> for CharacterMutation {
    fn from(gain_exalt_experience: GainExaltExperience) -> Self {
        CharacterMutation::GainExaltExperience(gain_exalt_experience)
    }
}