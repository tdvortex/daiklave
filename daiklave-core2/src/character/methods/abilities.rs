use crate::{
    abilities::{Abilities, AbilitiesVanilla, AbilityError, AbilityNameVanilla, AbilityRating},
    Character, CharacterMutationError,
};

impl<'view, 'source> Character<'source> {
    pub(crate) fn vanilla_abilities(&'view self) -> &'view AbilitiesVanilla<'source> {
        &self.abilities
    }

    /// Get read-only access to a character’s Abilities.
    pub fn abilities(&'view self) -> Abilities<'view, 'source> {
        Abilities(self)
    }

    /// Check if an ability's dots can be set to a specific level.
    pub fn check_set_ability_dots(
        &self,
        _ability_name: AbilityNameVanilla,
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

    /// Set an ability's dots to a specific dot value. If this sets the ability
    /// to 0 dots, will erase all specialties.
    pub fn set_ability_dots(
        &mut self,
        ability_name: AbilityNameVanilla,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_ability_dots(ability_name, dots)?;
        self.abilities.get_mut(ability_name).set_dots(dots)?;
        Ok(self)
    }

    /// Checks if a specialty can be added to an ability. Errors if ability is
    /// 0 dots or specialty is not unique.
    pub fn check_add_specialty(
        &self,
        ability_name: AbilityNameVanilla,
        specialty: &str,
    ) -> Result<(), CharacterMutationError> {
        if let AbilityRating::NonZero(_, specialties) = self.vanilla_abilities().get(ability_name) {
            if specialties.contains(specialty) {
                Err(CharacterMutationError::AbilityError(
                    AbilityError::DuplicateSpecialty,
                ))
            } else {
                Ok(())
            }
        } else {
            Err(CharacterMutationError::AbilityError(
                AbilityError::ZeroAbilitySpecialty,
            ))
        }
    }

    /// Adds a specialty to an ability.
    pub fn add_specialty(
        &mut self,
        ability_name: AbilityNameVanilla,
        specialty: &'source str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_add_specialty(ability_name, specialty)?;
        self.abilities
            .get_mut(ability_name)
            .add_specialty(specialty)?;
        Ok(self)
    }

    /// Checks if a specialty can be removed from an ability. Returns an error
    /// if specialty does not exist.
    pub fn check_remove_specialty(
        &self,
        ability_name: AbilityNameVanilla,
        specialty: &str,
    ) -> Result<(), CharacterMutationError> {
        if let AbilityRating::NonZero(_, specialties) = self.vanilla_abilities().get(ability_name) {
            if !specialties.contains(specialty) {
                Err(CharacterMutationError::AbilityError(
                    AbilityError::SpecialtyNotFound,
                ))
            } else {
                Ok(())
            }
        } else {
            Err(CharacterMutationError::AbilityError(
                AbilityError::SpecialtyNotFound,
            ))
        }
    }

    /// Removes a specialty from an ability.
    pub fn remove_specialty(
        &mut self,
        ability_name: AbilityNameVanilla,
        specialty: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_remove_specialty(ability_name, specialty)?;
        self.abilities
            .get_mut(ability_name)
            .remove_specialty(specialty)?;
        Ok(self)
    }
}