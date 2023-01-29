use std::num::NonZeroU16;

use crate::CharacterMutation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpendExaltExperience(pub NonZeroU16);

impl From<SpendExaltExperience> for CharacterMutation {
    fn from(spend_experience: SpendExaltExperience) -> Self {
        CharacterMutation::SpendExaltExperience(spend_experience)
    }
}