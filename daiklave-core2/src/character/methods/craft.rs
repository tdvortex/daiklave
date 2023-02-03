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

    /// Adds a specialty to a specific Craft ability.
    pub fn add_craft_specialty(
        &mut self,
        focus: &str,
        specialty: &'source str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.craft.0.get_mut(focus).ok_or(CharacterMutationError::AbilityError(AbilityError::ZeroAbilitySpecialty))?.add_specialty(specialty)?;
        Ok(self)
    }

    /// Removes a specialty from a specific Craft ability.
    pub fn remove_craft_specialty(
        &mut self,
        focus: &str,
        specialty: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.craft.0.get_mut(focus).ok_or(CharacterMutationError::AbilityError(AbilityError::SpecialtyNotFound))?.remove_specialty(specialty)?;
        Ok(self)
    }
}
