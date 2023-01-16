use crate::{
    abilities::AbilityNameVanilla,
    martial_arts::{MartialArts, MartialArtsError, MartialArtsStyle, MartialArtsStyleId},
    Character, CharacterMutationError,
};

impl<'view, 'source> Character<'source> {
    /// Accesses Martial Arts styles, abilities, and Charms.
    pub fn martial_arts(&'view self) -> MartialArts<'view, 'source> {
        MartialArts(&self.exaltation)
    }

    /// Checks if a Martial Arts style can be added to the character.
    pub fn check_add_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
        style: &MartialArtsStyle,
    ) -> Result<(), CharacterMutationError> {
        if self.abilities().get(AbilityNameVanilla::Brawl).dots() < 1 {
            return Err(CharacterMutationError::MartialArtsError(
                MartialArtsError::PrerequsitesNotMet,
            ));
        }

        self.exaltation.check_add_martial_arts_style(id, style)
    }

    /// Adds a Martial Arts style to the character.
    pub fn add_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
        style: &'source MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_add_martial_arts_style(id, style)?;
        self.exaltation.add_martial_arts_style(id, style)?;

        Ok(self)
    }

    /// Checks if a Martial Arts style can be removed from the character.
    pub fn check_remove_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
    ) -> Result<(), CharacterMutationError> {
        self.exaltation.check_remove_martial_arts_style(id)
    }

    /// Removes a Martial Arts style from the character.
    pub fn remove_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.remove_martial_arts_style(id)?;
        Ok(self)
    }

    /// Checks if the ability dots for the specified Martial Arts style
    /// can be set to a given value.
    pub fn check_set_martial_arts_dots(
        &self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<(), CharacterMutationError> {
        self.exaltation.check_set_martial_arts_dots(id, dots)
    }

    /// Sets the ability dots for a specific Martial Arts style.
    pub fn set_martial_arts_dots(
        &mut self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exaltation.set_martial_arts_dots(id, dots)?;
        Ok(self)
    }
}
