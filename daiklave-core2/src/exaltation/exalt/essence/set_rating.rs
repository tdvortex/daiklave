use std::num::NonZeroU8;

use crate::CharacterMutation;

use super::EssenceError;

/// A mutation to set the Essence rating of the character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetEssenceRating(pub(crate) NonZeroU8);

impl SetEssenceRating {
    /// Specifies the dot rating to set Essence to. Returns Err if the dot 
    /// rating would be above 5.
    pub fn dots(dots: NonZeroU8) -> Result<Self, EssenceError> {
        if dots > NonZeroU8::new(5).unwrap() {
            Err(EssenceError::InvalidRating)
        } else {
            Ok(Self(dots))
        }
    }
}

impl From<SetEssenceRating> for CharacterMutation {
    fn from(set_essence_rating: SetEssenceRating) -> Self {
        Self::SetEssenceRating(set_essence_rating)
    }
}