use crate::{
    attributes::{AttributeError, AttributeName, Attributes},
    Character, CharacterMutationError, abilities::AbilityNameVanilla,
};

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
        if !(1..=5).contains(&dots) {
            Err(CharacterMutationError::AttributeError(
                AttributeError::InvalidRating,
            ))
        } else {
            let old_dots = self.attributes().get(attribute_name).dots();
            self.attributes.set_dots(attribute_name, dots)?;
            if old_dots > dots {
                if attribute_name == AttributeName::Intelligence {
                    self.exaltation.correct_sorcery_level(self.abilities().get(AbilityNameVanilla::Occult).dots(), dots, self.essence().map_or(1, |essence| essence.rating()));
                }

                self.correct_merits();
            }
            Ok(self)
        }
    }
}
