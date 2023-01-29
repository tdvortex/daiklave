use std::num::NonZeroU16;

use crate::CharacterMutation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpendExperience(pub NonZeroU16);

impl From<SpendExperience> for CharacterMutation {
    fn from(spend_experience: SpendExperience) -> Self {
        CharacterMutation::SpendExperience(spend_experience)
    }
}