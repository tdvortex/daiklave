use std::num::NonZeroU8;

use crate::CharacterMutation;

use super::EssenceError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetEssenceRating(NonZeroU8);

impl SetEssenceRating {
    pub fn new(dots: NonZeroU8) -> Result<Self, EssenceError> {
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