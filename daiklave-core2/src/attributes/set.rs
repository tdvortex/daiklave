use std::num::NonZeroU8;

use crate::CharacterMutation;

use super::{AttributeError, AttributeName};

/// A mutation to set an attribute to a specific dot value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetAttribute {
    pub(crate) name: AttributeName,
    pub(crate) dots: NonZeroU8,
}

impl SetAttribute {
    /// Creates a new SetAttribute mutation. Returns Err if dots = 0 or dots > 5.
    pub fn new(name: AttributeName, dots: u8) -> Result<Self, AttributeError> {
        if dots > 5 {
            Err(AttributeError::InvalidRating)
        } else {
            Ok(Self {
                name,
                dots: NonZeroU8::new(dots).ok_or(AttributeError::InvalidRating)?,
            })
        }
    }
}

impl From<SetAttribute> for CharacterMutation {
    fn from(set_attribute: SetAttribute) -> Self {
        Self::SetAttribute(set_attribute)
    }
}
