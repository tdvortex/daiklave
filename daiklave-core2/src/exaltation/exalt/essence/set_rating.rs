use std::num::NonZeroU8;

use super::EssenceError;

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