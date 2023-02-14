use std::num::NonZeroU16;

use serde::{Serialize, Deserialize};

use crate::CharacterMutation;

/// A mutation to spend experience. This reduces
/// current experience and leaves total unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpendExperience(pub NonZeroU16);

impl From<SpendExperience> for CharacterMutation {
    fn from(spend_experience: SpendExperience) -> Self {
        CharacterMutation::SpendExperience(spend_experience)
    }
}
