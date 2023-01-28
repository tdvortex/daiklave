use std::num::NonZeroU8;

use super::{AttributeName, AttributeError};

pub struct SetAttribute {
    name: AttributeName,
    dots: NonZeroU8
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