use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    abilities::Ability,
    martial_arts::{
        AddMartialArtsStyleError, MartialArtsStyle, MartialArtsStyleId,
        RemoveMartialArtsStyleError, SetMartialArtsDotsError,
    },
    CharacterMutationError, sorcery::circles::terrestrial::sorcerer::TerrestrialCircleSorcerer,
};

use super::martial_arts::MortalMartialArtist;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub(crate) struct Mortal {
    martial_arts_styles: HashMap<MartialArtsStyleId, MortalMartialArtist>,
    sorcery: Option<TerrestrialCircleSorcerer>,
}

impl Mortal {
    pub fn new(
        martial_arts_styles: HashMap<MartialArtsStyleId, MortalMartialArtist>,
        sorcery: Option<TerrestrialCircleSorcerer>,
    ) -> Self {
        Self {
            martial_arts_styles,
            sorcery,
        }
    }

    pub fn martial_arts_styles(&self) -> &HashMap<MartialArtsStyleId, MortalMartialArtist> {
        &self.martial_arts_styles
    }

    pub fn martial_arts_styles_mut(
        &mut self,
    ) -> &mut HashMap<MartialArtsStyleId, MortalMartialArtist> {
        &mut self.martial_arts_styles
    }

    pub fn sorcery(&self) -> Option<&TerrestrialCircleSorcerer> {
        self.sorcery.as_ref()
    }

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
        _dots: u8,
    ) -> Result<(), CharacterMutationError> {
        if self.martial_arts_styles.contains_key(&id) {
            Ok(())
        } else {
            Err(CharacterMutationError::SetMartialArtsDotsError(
                SetMartialArtsDotsError::NotFound,
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
            Err(CharacterMutationError::SetMartialArtsDotsError(
                SetMartialArtsDotsError::NotFound,
            ))
        }
    }
}
