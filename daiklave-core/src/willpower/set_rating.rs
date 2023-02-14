use std::num::NonZeroU8;

use serde::{Deserialize, Serialize};

use crate::CharacterMutation;

use super::WillpowerError;

/// A mutation to set the character's willpower rating.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetWillpowerRating(pub(crate) NonZeroU8);

impl SetWillpowerRating {
    /// Creates a new mutation to set the character's willpower rating.
    pub fn new(rating: NonZeroU8) -> Result<Self, WillpowerError> {
        if rating > NonZeroU8::new(10).unwrap() {
            Err(WillpowerError::InvalidRating)
        } else {
            Ok(SetWillpowerRating(rating))
        }
    }
}

impl From<SetWillpowerRating> for CharacterMutation {
    fn from(set_willpower_rating: SetWillpowerRating) -> Self {
        Self::SetWillpowerRating(set_willpower_rating)
    }
}
