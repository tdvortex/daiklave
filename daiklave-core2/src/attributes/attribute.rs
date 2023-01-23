use std::num::NonZeroU8;

use super::{category::AttributeCategory, AttributeName};

/// One attribute of a character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Attribute {
    pub(crate) name: AttributeName,
    pub(crate) dots: NonZeroU8,
}

impl Attribute {
    /// The name of the attribute.
    pub fn name(&self) -> AttributeName {
        self.name
    }

    /// The number of dots in the attribute. Always at least 1, up to 5.
    pub fn dots(&self) -> u8 {
        self.dots.get()
    }

    /// The attribute's category (Mental/Social/Physical).
    pub fn category(&self) -> AttributeCategory {
        self.name.into()
    }
}
