use crate::{abilities::AbilityError, craft::Craft, Character, CharacterMutationError};

impl<'view, 'source> Character<'source> {
    /// The character's Craft abilities and specialties.
    pub fn craft(&'view self) -> &'view Craft<'source> {
        &self.craft
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
