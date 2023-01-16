use crate::{Character, attributes::{Attributes, AttributeName, AttributeError}, CharacterMutationError};

impl<'source> Character<'source> {
    /// Gets a struct reference for the character's attributes.
    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    /// Validates that the requested dot level is an appropriate attribute
    /// rating. Attributes must be between 1 and 5 for all player characters.
    pub fn check_set_attribute(
        &self,
        _attribute_name: AttributeName,
        dots: u8,
    ) -> Result<(), CharacterMutationError> {
        if !(1..=5).contains(&dots) {
            Err(CharacterMutationError::AttributeError(
                AttributeError::InvalidRating,
            ))
        } else {
            Ok(())
        }
    }

    /// Sets the specified attribute name to the specified dot rating.
    pub fn set_attribute(
        &mut self,
        attribute_name: AttributeName,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_attribute(attribute_name, dots)?;
        self.attributes.set_dots(attribute_name, dots)?;
        Ok(self)
    }
}

