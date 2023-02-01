use std::num::NonZeroU8;

use crate::CharacterMutation;

use super::{AttributeName, AttributeError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetAttribute {
    pub(crate) name: AttributeName,
    pub(crate) dots: NonZeroU8
}

impl SetAttribute {
    pub fn new(name: AttributeName, dots: NonZeroU8) -> Result<Self, AttributeError> {
        if dots > NonZeroU8::new(5).unwrap() {
            Err(AttributeError::InvalidRating)
        } else {
            Ok(Self {
                name,
                dots,
            })
        }
    }
}

impl From<SetAttribute> for CharacterMutation {
    fn from(set_attribute: SetAttribute) -> Self {
        Self::SetAttribute(set_attribute)
    }
}