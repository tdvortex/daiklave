use std::num::NonZeroU16;

use crate::CharacterMutation;

pub struct SpendExperience(NonZeroU16);

impl SpendExperience {
    pub fn new(amount: NonZeroU16) -> Self {
        Self(amount)
    }
}

impl From<SpendExperience> for CharacterMutation {
    fn from(spend_experience: SpendExperience) -> Self {
        CharacterMutation::SpendExperience(spend_experience)
    }
}