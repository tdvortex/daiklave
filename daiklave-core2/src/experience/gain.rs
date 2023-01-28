use std::num::NonZeroU16;

use crate::CharacterMutation;

pub struct GainExperience(NonZeroU16);

impl GainExperience {
    pub fn new(amount: NonZeroU16) -> Self {
        Self(amount)
    }
}

impl From<GainExperience> for CharacterMutation {
    fn from(gain_experience: GainExperience) -> Self {
        CharacterMutation::GainExperience(gain_experience)
    }
}