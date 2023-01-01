use serde::{Deserialize, Serialize};

use crate::{
    martial_arts::{MartialArtist, MartialArtsStyle, MartialArtsStyleId},
    sorcery::SolarSorcerer,
    CharacterMutationError,
};

use super::{
    exalt::{
        essence::{
            CommitMotesError, Essence, MoteCommitmentId, MotePoolName, RecoverMotesError,
            SetEssenceRatingError, SpendMotesError, UncommitMotesError,
        },
        exalt_type::{solar::Solar, ExaltType},
        Exalt,
    },
    martial_arts::ExaltationMartialArtist,
    mortal::Mortal,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Exaltation {
    Mortal(Box<Mortal>),
    Exalt(Box<Exalt>),
}

impl Default for Exaltation {
    fn default() -> Self {
        Self::Mortal(Box::new(Mortal::default()))
    }
}

impl Exaltation {
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

        let exalt = if let Exaltation::Exalt(exalt) = self {
            exalt
        } else {
            unreachable!()
        };

        // Preserve Terrestrial circle sorcery
        let sorcery = {
            match exalt.exalt_type() {
                ExaltType::Solar(solar) => {
                    if let Some(sorcery) = &solar.sorcery {
                        match sorcery {
                            SolarSorcerer::Terrestrial(terrestrial) => {
                                Some((**terrestrial).clone())
                            }
                            SolarSorcerer::Celestial(celestial) => {
                                Some((**celestial).clone().into())
                            }
                            SolarSorcerer::Solar(solar) => Some((**solar).clone().into()),
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

        *self = Exaltation::Mortal(Box::new(Mortal::new(martial_arts_styles, sorcery)));

        Ok(self)
    }

    pub fn essence(&self) -> Option<&Essence> {
        match self {
            Exaltation::Mortal(_) => None,
            Exaltation::Exalt(exalt) => Some(exalt.essence()),
        }
    }

    pub fn check_spend_motes(
        &self,
        first: MotePoolName,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::MortalError,
            )),
            Exaltation::Exalt(exalt) => exalt.check_spend_motes(first, amount),
        }
    }

    pub fn spend_motes(
        &mut self,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::MortalError,
            )),
            Exaltation::Exalt(exalt) => exalt.spend_motes(first, amount),
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
            Exaltation::Mortal(_) => Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::MortalError,
            )),
            Exaltation::Exalt(exalt) => exalt.check_commit_motes(id, name, first, amount),
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
            Exaltation::Mortal(_) => Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::MortalError,
            )),
            Exaltation::Exalt(exalt) => exalt.commit_motes(id, name, first, amount),
        }?;
        Ok(self)
    }

    pub fn check_recover_motes(&self, _amount: u8) -> Result<(), CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            Exaltation::Exalt(_) => Ok(()),
        }
    }

    pub fn recover_motes(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            Exaltation::Exalt(exalt_type) => exalt_type.recover_motes(amount),
        }?;
        Ok(self)
    }

    pub fn check_uncommit_motes(
        &self,
        id: &MoteCommitmentId,
    ) -> Result<(), CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            Exaltation::Exalt(exalt_type) => exalt_type.check_uncommit_motes(id),
        }
    }

    pub fn uncommit_motes(
        &mut self,
        id: &MoteCommitmentId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            Exaltation::Exalt(exalt_type) => exalt_type.uncommit_motes(id),
        }?;
        Ok(self)
    }

    pub fn check_set_essence_rating(&self, rating: u8) -> Result<(), CharacterMutationError> {
        match self {
            Exaltation::Mortal(_) => Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::MortalError,
            )),
            Exaltation::Exalt(_) => {
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
            Exaltation::Exalt(exalt_type) => exalt_type.set_essence_rating(rating),
            Exaltation::Mortal(_) => Err(CharacterMutationError::SetEssenceRatingError(
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
            Exaltation::Mortal(mortal) => mortal.check_add_martial_arts_style(id, style),
            Exaltation::Exalt(exalt) => exalt.check_add_martial_arts_style(id, style),
        }
    }

    pub(crate) fn add_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
        style: &MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.add_martial_arts_style(id, style)?;
            }
            Exaltation::Exalt(exalt) => {
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
            Exaltation::Mortal(mortal) => mortal.check_remove_martial_arts_style(id),
            Exaltation::Exalt(exalt) => exalt.check_remove_martial_arts_style(id),
        }
    }

    pub(crate) fn remove_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.remove_martial_arts_style(id)?;
            }
            Exaltation::Exalt(exalt) => {
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
            Exaltation::Mortal(mortal) => mortal.check_set_martial_arts_dots(id, dots),
            Exaltation::Exalt(exalt) => exalt.check_set_martial_arts_dots(id, dots),
        }
    }

    pub(crate) fn set_martial_arts_dots(
        &mut self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            Exaltation::Mortal(mortal) => {
                mortal.set_martial_arts_dots(id, dots)?;
            }
            Exaltation::Exalt(exalt) => {
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

    pub fn solar_traits(&self) -> Option<&Solar> {
        if let Self::Exalt(exalt) = self {
            exalt.solar_traits()
        } else {
            None
        }
    }

    pub fn check_set_solar(&self, _solar: &Solar) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_solar(&mut self, solar: &Solar) -> Result<&mut Self, CharacterMutationError> {
        if self.is_solar() {
            return Ok(self);
        }

        match self {
            Exaltation::Mortal(mortal) => {
                // Default to essence 1
                // Preserve martial arts styles, with empty Charms set
                *self = Self::Exalt(Box::new(Exalt::new(
                    Essence::new_solar(1),
                    std::mem::take(mortal.as_mut().martial_arts_styles_mut())
                        .into_iter()
                        .map(|(id, mortal_artist)| (id, mortal_artist.into()))
                        .collect(),
                    ExaltType::Solar(solar.clone()),
                )))
            }
            Exaltation::Exalt(exalt) => {
                // Preserve essence rating
                // Preserve martial arts styles (including charms)
                *self = Self::Exalt(Box::new(Exalt::new(
                    Essence::new_solar(exalt.essence().rating()),
                    std::mem::take(exalt.martial_arts_styles_mut()),
                    ExaltType::Solar(solar.clone()),
                )));
            }
        }

        Ok(self)
    }
}

impl<'char> Exaltation {
    pub(crate) fn martial_artist(
        &'char self,
        id: MartialArtsStyleId,
    ) -> Option<MartialArtist<'char>> {
        match self {
            Exaltation::Mortal(mortal) => Some(MartialArtist(ExaltationMartialArtist::Mortal(
                mortal.martial_arts_styles().get(&id)?,
            ))),
            Exaltation::Exalt(exalt) => Some(MartialArtist(ExaltationMartialArtist::Exalt(
                exalt.martial_arts_styles().get(&id)?,
            ))),
        }
    }

    pub(crate) fn martial_arts_id_iter(&'char self) -> impl Iterator<Item = MartialArtsStyleId> {
        match self {
            Exaltation::Mortal(mortal) => mortal
                .martial_arts_styles()
                .keys()
                .copied()
                .collect::<Vec<MartialArtsStyleId>>()
                .into_iter(),
            Exaltation::Exalt(exalt) => exalt
                .martial_arts_styles()
                .keys()
                .copied()
                .collect::<Vec<MartialArtsStyleId>>()
                .into_iter(),
        }
    }
}
