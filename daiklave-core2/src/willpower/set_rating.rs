use std::num::NonZeroU8;

use crate::CharacterMutation;

use super::WillpowerError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetWillpowerRating(pub(crate) NonZeroU8);

impl SetWillpowerRating {
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
