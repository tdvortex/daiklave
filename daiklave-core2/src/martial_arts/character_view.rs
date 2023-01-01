use crate::{abilities::AbilityNameVanilla, CharacterMutationError, CharacterView};

use super::{AddMartialArtsStyleError, MartialArtsStyle, MartialArtsStyleId, MartialArtsView};

impl<'source> CharacterView<'source> {
    /// Checks if a Martial Arts style can be added to the character.
    pub fn check_add_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
        style: &MartialArtsStyle,
    ) -> Result<(), CharacterMutationError> {
        if self.abilities().dots(AbilityNameVanilla::Brawl) < 1 {
            return Err(CharacterMutationError::AddMartialArtsStyleError(
                AddMartialArtsStyleError::PrerequsitesNotMet(
                    "Brawl must be 1+ to take Martial Artist merit".to_owned(),
                ),
            ));
        }

        self.exalt_state.check_add_martial_arts_style(id, style)
    }

    /// Adds a Martial Arts style to the character.
    pub fn add_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
        style: &'source MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_add_martial_arts_style(id, style)?;
        self.exalt_state.add_martial_arts_style(id, style)?;

        Ok(self)
    }

    /// Checks if a Martial Arts style can be removed from the character.
    pub fn check_remove_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
    ) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_remove_martial_arts_style(id)
    }

    /// Removes a Martial Arts style from the character.
    pub fn remove_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.remove_martial_arts_style(id)?;
        Ok(self)
    }

    pub fn check_set_martial_arts_dots(
        &self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_martial_arts_dots(id, dots)
    }

    pub fn set_martial_arts_dots(
        &mut self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.set_martial_arts_dots(id, dots)?;
        Ok(self)
    }
}

impl<'view, 'source> CharacterView<'source> {
    pub fn martial_arts(&'view self) -> MartialArtsView<'view, 'source> {
        MartialArtsView(self)
    }
}
