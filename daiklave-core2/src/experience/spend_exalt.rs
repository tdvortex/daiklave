use std::num::NonZeroU16;

use crate::CharacterMutation;

pub struct SpendExaltExperience(NonZeroU16);

impl SpendExaltExperience {
    pub fn new(amount: NonZeroU16) -> Self {
        Self(amount)
    }
}

impl From<SpendExaltExperience> for CharacterMutation {
    fn from(spend_experience: SpendExaltExperience) -> Self {
        CharacterMutation::SpendExaltExperience(spend_experience)
    }
}