use serde::{Deserialize, Serialize};

use crate::{
    martial_arts::{MartialArtistMemo, MartialArtsStyle, MartialArtsStyleId},
    sorcery::{SorceryMemo},
    CharacterMutationError,
};

use super::{
    exalt::{
        essence::{
            CommitMotesError, EssenceMemo, MoteCommitmentId, MotePoolName, RecoverMotesError,
            SetEssenceRatingError, SpendMotesError, UncommitMotesError,
        },
        exalt_type::{solar::{SolarMemo, SolarSorcererMemo}, ExaltTypeMemo},
        ExaltMemo,
    },
    martial_arts::ExaltationMartialArtistMemo,
    mortal::MortalMemo, sorcery::SorcerySwitchMemo,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltationMemo {
    Mortal(Box<MortalMemo>),
    Exalt(Box<ExaltMemo>),
}

impl Default for ExaltationMemo {
    fn default() -> Self {
        Self::Mortal(Box::new(MortalMemo::default()))
    }
}

impl ExaltationMemo {
    pub fn is_mortal(&self) -> bool {
        matches!(self, Self::Mortal(_))
    }

    pub fn is_exalted(&self) -> bool {
        !self.is_mortal()
    }

    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if self.is_mortal() {
            return Ok(self);
        }

        let exalt = if let ExaltationMemo::Exalt(exalt) = self {
            exalt
        } else {
            unreachable!()
        };

        // Preserve Terrestrial circle sorcery
        let sorcery = {
            match exalt.exalt_type() {
                ExaltTypeMemo::Solar(solar) => {
                    if let Some(sorcery) = &solar.sorcery {
                        match sorcery {
                            SolarSorcererMemo::Terrestrial(terrestrial) => {
                                Some((**terrestrial).clone())
                            }
                            SolarSorcererMemo::Celestial(celestial) => {
                                Some((**celestial).clone().into())
                            }
                            SolarSorcererMemo::Solar(solar) => Some((**solar).clone().into()),
                        }
                    } else {
                        None
                    }
                }
            }
        };

        // Preserve martial arts styles
        let martial_arts_styles = std::mem::take(exalt.martial_arts_styles_mut())
            .into_iter()
            .map(|(id, exalt_artist)| (id, exalt_artist.into()))
            .collect();

        *self = ExaltationMemo::Mortal(Box::new(MortalMemo::new(martial_arts_styles, sorcery)));

        Ok(self)
    }

    pub fn essence(&self) -> Option<&EssenceMemo> {
        match self {
            ExaltationMemo::Mortal(_) => None,
            ExaltationMemo::Exalt(exalt) => Some(exalt.essence()),
        }
    }

    pub fn check_spend_motes(
        &self,
        first: MotePoolName,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltationMemo::Mortal(_) => Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::MortalError,
            )),
            ExaltationMemo::Exalt(exalt) => exalt.check_spend_motes(first, amount),
        }
    }

    pub fn spend_motes(
        &mut self,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltationMemo::Mortal(_) => Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::MortalError,
            )),
            ExaltationMemo::Exalt(exalt) => exalt.spend_motes(first, amount),
        }?;
        Ok(self)
    }

    pub fn check_commit_motes(
        &self,
        id: &MoteCommitmentId,
        name: &str,
        first: MotePoolName,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltationMemo::Mortal(_) => Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::MortalError,
            )),
            ExaltationMemo::Exalt(exalt) => exalt.check_commit_motes(id, name, first, amount),
        }
    }

    pub fn commit_motes(
        &mut self,
        id: &MoteCommitmentId,
        name: &str,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltationMemo::Mortal(_) => Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::MortalError,
            )),
            ExaltationMemo::Exalt(exalt) => exalt.commit_motes(id, name, first, amount),
        }?;
        Ok(self)
    }

    pub fn check_recover_motes(&self, _amount: u8) -> Result<(), CharacterMutationError> {
        match self {
            ExaltationMemo::Mortal(_) => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            ExaltationMemo::Exalt(_) => Ok(()),
        }
    }

    pub fn recover_motes(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltationMemo::Mortal(_) => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            ExaltationMemo::Exalt(exalt_type) => exalt_type.recover_motes(amount),
        }?;
        Ok(self)
    }

    pub fn check_uncommit_motes(
        &self,
        id: &MoteCommitmentId,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltationMemo::Mortal(_) => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            ExaltationMemo::Exalt(exalt_type) => exalt_type.check_uncommit_motes(id),
        }
    }

    pub fn uncommit_motes(
        &mut self,
        id: &MoteCommitmentId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltationMemo::Mortal(_) => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            ExaltationMemo::Exalt(exalt_type) => exalt_type.uncommit_motes(id),
        }?;
        Ok(self)
    }

    pub fn check_set_essence_rating(&self, rating: u8) -> Result<(), CharacterMutationError> {
        match self {
            ExaltationMemo::Mortal(_) => Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::MortalError,
            )),
            ExaltationMemo::Exalt(_) => {
                if (1..=5).contains(&rating) {
                    Ok(())
                } else {
                    Err(CharacterMutationError::SetEssenceRatingError(
                        SetEssenceRatingError::InvalidRating(rating),
                    ))
                }
            }
        }
    }

    pub fn set_essence_rating(&mut self, rating: u8) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_essence_rating(rating)?;
        match self {
            ExaltationMemo::Exalt(exalt_type) => exalt_type.set_essence_rating(rating),
            ExaltationMemo::Mortal(_) => Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::MortalError,
            )),
        }?;
        Ok(self)
    }

    pub(crate) fn check_add_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
        style: &MartialArtsStyle,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltationMemo::Mortal(mortal) => mortal.check_add_martial_arts_style(id, style),
            ExaltationMemo::Exalt(exalt) => exalt.check_add_martial_arts_style(id, style),
        }
    }

    pub(crate) fn add_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
        style: &MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltationMemo::Mortal(mortal) => {
                mortal.add_martial_arts_style(id, style)?;
            }
            ExaltationMemo::Exalt(exalt) => {
                exalt.add_martial_arts_style(id, style)?;
            }
        }
        Ok(self)
    }

    pub(crate) fn check_remove_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltationMemo::Mortal(mortal) => mortal.check_remove_martial_arts_style(id),
            ExaltationMemo::Exalt(exalt) => exalt.check_remove_martial_arts_style(id),
        }
    }

    pub(crate) fn remove_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltationMemo::Mortal(mortal) => {
                mortal.remove_martial_arts_style(id)?;
            }
            ExaltationMemo::Exalt(exalt) => {
                exalt.remove_martial_arts_style(id)?;
            }
        }
        Ok(self)
    }

    pub(crate) fn check_set_martial_arts_dots(
        &self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltationMemo::Mortal(mortal) => mortal.check_set_martial_arts_dots(id, dots),
            ExaltationMemo::Exalt(exalt) => exalt.check_set_martial_arts_dots(id, dots),
        }
    }

    pub(crate) fn set_martial_arts_dots(
        &mut self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltationMemo::Mortal(mortal) => {
                mortal.set_martial_arts_dots(id, dots)?;
            }
            ExaltationMemo::Exalt(exalt) => {
                exalt.set_martial_arts_dots(id, dots)?;
            }
        }
        Ok(self)
    }

    pub fn is_solar(&self) -> bool {
        if let Self::Exalt(exalt) = self {
            exalt.is_solar()
        } else {
            false
        }
    }

    pub fn solar_traits(&self) -> Option<&SolarMemo> {
        if let Self::Exalt(exalt) = self {
            exalt.solar_traits()
        } else {
            None
        }
    }

    pub fn check_set_solar(&self, _solar: &SolarMemo) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_solar(&mut self, solar: &SolarMemo) -> Result<&mut Self, CharacterMutationError> {
        if self.is_solar() {
            return Ok(self);
        }

        match self {
            ExaltationMemo::Mortal(mortal) => {
                // Default to essence 1
                // Preserve martial arts styles, with empty Charms set
                *self = Self::Exalt(Box::new(ExaltMemo::new(
                    EssenceMemo::new_solar(1),
                    std::mem::take(mortal.as_mut().martial_arts_styles_mut())
                        .into_iter()
                        .map(|(id, mortal_artist)| (id, mortal_artist.into()))
                        .collect(),
                    ExaltTypeMemo::Solar(solar.clone()),
                )))
            }
            ExaltationMemo::Exalt(exalt) => {
                // Preserve essence rating
                // Preserve martial arts styles (including charms)
                *self = Self::Exalt(Box::new(ExaltMemo::new(
                    EssenceMemo::new_solar(exalt.essence().rating()),
                    std::mem::take(exalt.martial_arts_styles_mut()),
                    ExaltTypeMemo::Solar(solar.clone()),
                )));
            }
        }

        Ok(self)
    }
}

impl<'char> ExaltationMemo {
    pub(crate) fn martial_artist(
        &'char self,
        id: MartialArtsStyleId,
    ) -> Option<MartialArtistMemo<'char>> {
        match self {
            ExaltationMemo::Mortal(mortal) => Some(MartialArtistMemo(ExaltationMartialArtistMemo::Mortal(
                mortal.martial_arts_styles().get(&id)?,
            ))),
            ExaltationMemo::Exalt(exalt) => Some(MartialArtistMemo(ExaltationMartialArtistMemo::Exalt(
                exalt.martial_arts_styles().get(&id)?,
            ))),
        }
    }

    pub(crate) fn martial_arts_id_iter(&'char self) -> impl Iterator<Item = MartialArtsStyleId> {
        match self {
            ExaltationMemo::Mortal(mortal) => mortal
                .martial_arts_styles()
                .keys()
                .copied()
                .collect::<Vec<MartialArtsStyleId>>()
                .into_iter(),
            ExaltationMemo::Exalt(exalt) => exalt
                .martial_arts_styles()
                .keys()
                .copied()
                .collect::<Vec<MartialArtsStyleId>>()
                .into_iter(),
        }
    }

    pub(crate) fn sorcery(&'char self) -> Option<SorceryMemo<'char>> {
        match self {
            ExaltationMemo::Mortal(mortal) => mortal
                .sorcery()
                .map(|terrestrial| SorceryMemo(SorcerySwitchMemo::Mortal(terrestrial))),
            ExaltationMemo::Exalt(exalt) => exalt.sorcery(),
        }
    }
}