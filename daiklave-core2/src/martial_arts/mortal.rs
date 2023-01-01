use serde::{Deserialize, Serialize};

use crate::{
    abilities::{Ability, AbilityView},
    exalt_state::mortal::{Mortal, MortalView},
    CharacterMutationError,
};

use super::{
    AddMartialArtsStyleError, MartialArtsStyle, MartialArtsStyleId, RemoveMartialArtsStyleError,
    SetMartialArtsError,
};

impl Mortal {
    pub(crate) fn check_add_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
        _style: &MartialArtsStyle,
    ) -> Result<(), CharacterMutationError> {
        if self.martial_arts_styles.contains_key(&id) {
            Err(CharacterMutationError::AddMartialArtsStyleError(
                AddMartialArtsStyleError::DuplicateStyle,
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn add_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
        style: &MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_add_martial_arts_style(id, style)?;
        self.martial_arts_styles.insert(
            id,
            MortalMartialArtist {
                style: style.to_owned(),
                ability: Ability::Zero,
            },
        );
        Ok(self)
    }

    pub(crate) fn check_remove_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
    ) -> Result<(), CharacterMutationError> {
        if self.martial_arts_styles.contains_key(&id) {
            Err(CharacterMutationError::RemoveMartialArtsStyleError(
                RemoveMartialArtsStyleError::NotFound,
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn remove_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_remove_martial_arts_style(id)?;
        self.martial_arts_styles.remove(&id);
        Ok(self)
    }

    pub(crate) fn check_set_martial_arts_dots(
        &self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<(), CharacterMutationError> {
        if let Some(style) = self.martial_arts_styles.get(&id) {
            Ok(())
        } else {
            Err(CharacterMutationError::SetMartialArtsError(
                SetMartialArtsError::NotFound,
            ))
        }
    }

    pub(crate) fn set_martial_arts_dots(
        &mut self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Some(style) = self.martial_arts_styles.get_mut(&id) {
            // Mortals have no charms to lose if dots are zero
            Ok(self)
        } else {
            Err(CharacterMutationError::SetMartialArtsError(
                SetMartialArtsError::NotFound,
            ))
        }
    }
}

impl<'source> MortalView<'source> {
    pub(crate) fn check_add_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
        _style: &MartialArtsStyle,
    ) -> Result<(), CharacterMutationError> {
        if self.martial_arts_styles.contains_key(&id) {
            Err(CharacterMutationError::AddMartialArtsStyleError(
                AddMartialArtsStyleError::DuplicateStyle,
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn add_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
        style: &'source MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_add_martial_arts_style(id, style)?;
        self.martial_arts_styles.insert(
            id,
            MortalMartialArtistView {
                style,
                ability: AbilityView::Zero,
            },
        );
        Ok(self)
    }

    pub(crate) fn check_remove_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
    ) -> Result<(), CharacterMutationError> {
        if !self.martial_arts_styles.contains_key(&id) {
            Err(CharacterMutationError::RemoveMartialArtsStyleError(
                RemoveMartialArtsStyleError::NotFound,
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn remove_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_remove_martial_arts_style(id)?;
        self.martial_arts_styles.remove(&id);
        Ok(self)
    }

    pub(crate) fn check_set_martial_arts_dots(
        &self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<(), CharacterMutationError> {
        if let Some(style) = self.martial_arts_styles.get(&id) {
            Ok(())
        } else {
            Err(CharacterMutationError::SetMartialArtsError(
                SetMartialArtsError::NotFound,
            ))
        }
    }

    pub(crate) fn set_martial_arts_dots(
        &mut self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Some(style) = self.martial_arts_styles.get_mut(&id) {
            // Mortals have no charms to lose if dots are zero
            style.ability.set_dots(dots)?;
            Ok(self)
        } else {
            Err(CharacterMutationError::SetMartialArtsError(
                SetMartialArtsError::NotFound,
            ))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalMartialArtist {
    pub style: MartialArtsStyle,
    pub ability: Ability,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MortalMartialArtistView<'source> {
    pub style: &'source MartialArtsStyle,
    pub ability: AbilityView<'source>,
}
