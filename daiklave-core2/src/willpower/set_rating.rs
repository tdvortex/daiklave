use std::num::NonZeroU8;

use super::WillpowerError;

pub struct SetWillpowerRating(NonZeroU8);

impl SetWillpowerRating {
    pub fn new(rating: NonZeroU8) -> Result<Self, WillpowerError> {
        if rating > NonZeroU8::new(10).unwrap() {
            Err(WillpowerError::InvalidRating)
        } else {
            Ok(SetWillpowerRating(rating))
        }
    }
}