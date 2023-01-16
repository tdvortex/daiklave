use crate::{Character, CharacterMutationError, abilities::AbilityError, craft::Craft};

impl<'view, 'source> Character<'source> {
    /// The character's Craft abilities and specialties.
    pub fn craft(&'view self) -> &'view Craft<'source> {
        &self.craft
    }

    /// Checks if a Craft ability can be set to the specified dots.
    pub fn check_set_craft_dots(
        &self,
        _focus: &str,
        dots: u8,
    ) -> Result<(), CharacterMutationError> {
        if dots > 5 {
            Err(CharacterMutationError::AbilityError(
                AbilityError::InvalidRating,
            ))
        } else {
            Ok(())
        }
    }

    /// Sets a specific Craft focus area to the specified dots.
    pub fn set_craft_dots(
        &mut self,
        focus: &'source str,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if dots > 5 {
            Err(CharacterMutationError::AbilityError(
                AbilityError::InvalidRating,
            ))
        } else {
            self.craft.set_dots(focus, dots)?;
            Ok(self)
        }
    }
}