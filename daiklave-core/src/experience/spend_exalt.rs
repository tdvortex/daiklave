use std::num::NonZeroU16;

use serde::{Serialize, Deserialize};

use crate::CharacterMutation;

/// A mutation to spend Exalt experience (e.g. Solar Experience). This reduces
/// current experience and leaves total unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpendExaltExperience(pub NonZeroU16);

impl From<SpendExaltExperience> for CharacterMutation {
    fn from(spend_experience: SpendExaltExperience) -> Self {
        CharacterMutation::SpendExaltExperience(spend_experience)
    }
}
