use crate::{
    exaltation::{
        exalt::{
            essence::{
                CommitMotesError, EssenceView, MoteCommitmentId, MotePoolName, RecoverMotesError,
                SetEssenceRatingError, SpendMotesError, UncommitMotesError,
            },
            exalt_type::{ExaltType, ExaltTypeView},
            exalt_view::ExaltView,
        },
        ExaltStateView,
    },
    CharacterMutationError,
};

use super::{solar::Solar, solar_view::SolarView};

impl ExaltType {
    pub fn is_solar(&self) -> bool {
        true
    }

    pub fn solar_traits(&self) -> Option<&Solar> {
        match self {
            ExaltType::Solar(solar_traits) => Some(solar_traits),
        }
    }
}

impl<'source> ExaltTypeView<'source> {
    pub fn is_solar(&self) -> bool {
        true
    }

    pub fn solar_traits(&self) -> Option<&SolarView> {
        match self {
            ExaltTypeView::Solar(solar_traits) => Some(solar_traits),
        }
    }
}

impl<'source> ExaltStateView<'source> {
    pub fn is_solar(&self) -> bool {
        if let Self::Exalt(exalt_type) = self {
            exalt_type.is_solar()
        } else {
            false
        }
    }

    pub fn solar_traits(&self) -> Option<&SolarView> {
        if let Self::Exalt(exalt_type) = self {
            exalt_type.solar_traits()
        } else {
            None
        }
    }

    pub fn check_set_solar(&self, _solar: &'source Solar) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn check_set_solar_view(&self, _solar: &SolarView) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_solar(
        &mut self,
        solar: &'source Solar,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.is_solar() {
            return Ok(self);
        }

        self.set_solar_view(solar.as_view())
    }

    pub fn set_solar_view(
        &mut self,
        solar: SolarView<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.is_solar() {
            return Ok(self);
        }

        match self {
            ExaltStateView::Mortal(mortal) => {
                // Default to essence 1
                // Preserve martial arts styles, with empty Charms set
                *self = Self::Exalt(Box::new(ExaltView::new(
                    EssenceView::new_solar(1),
                    std::mem::take(&mut mortal.martial_arts_styles)
                        .into_iter()
                        .map(|(id, mortal_artist)| (id, mortal_artist.into()))
                        .collect(),
                    ExaltTypeView::Solar(solar),
                )))
            }
            ExaltStateView::Exalt(exalt) => {
                // Preserve essence rating
                // Preserve martial arts styles (including charms)
                *self = Self::Exalt(Box::new(ExaltView::new(
                    EssenceView::new_solar(exalt.essence().rating()),
                    std::mem::take(exalt.martial_arts_styles_mut()),
                    ExaltTypeView::Solar(solar),
                )));
            }
        }

        Ok(self)
    }

    pub fn essence(&self) -> Option<&EssenceView> {
        match self {
            ExaltStateView::Mortal(_) => None,
            ExaltStateView::Exalt(exalt) => Some(exalt.essence()),
        }
    }

    pub fn check_spend_motes(
        &self,
        first: MotePoolName,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(_) => Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::MortalError,
            )),
            ExaltStateView::Exalt(exalt) => exalt.check_spend_motes(first, amount),
        }
    }

    pub fn spend_motes(
        &mut self,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(_) => Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::MortalError,
            )),
            ExaltStateView::Exalt(exalt) => exalt.spend_motes(first, amount),
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
            ExaltStateView::Mortal(_) => Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::MortalError,
            )),
            ExaltStateView::Exalt(exalt) => exalt.check_commit_motes(id, name, first, amount),
        }
    }

    pub fn commit_motes(
        &mut self,
        id: &MoteCommitmentId,
        name: &'source str,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(_) => Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::MortalError,
            )),
            ExaltStateView::Exalt(exalt) => exalt.commit_motes(id, name, first, amount),
        }?;
        Ok(self)
    }

    pub fn check_recover_motes(&self, _amount: u8) -> Result<(), CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(_) => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            ExaltStateView::Exalt(_) => Ok(()),
        }
    }

    pub fn recover_motes(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(_) => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            ExaltStateView::Exalt(exalt) => exalt.recover_motes(amount),
        }?;
        Ok(self)
    }

    pub fn check_uncommit_motes(
        &self,
        id: &MoteCommitmentId,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(_) => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            ExaltStateView::Exalt(exalt) => exalt.check_uncommit_motes(id),
        }
    }

    pub fn uncommit_motes(
        &mut self,
        id: &MoteCommitmentId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(_) => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            ExaltStateView::Exalt(exalt) => exalt.uncommit_motes(id),
        }?;
        Ok(self)
    }

    pub fn check_set_essence_rating(&self, rating: u8) -> Result<(), CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(_) => Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::MortalError,
            )),
            ExaltStateView::Exalt(_) => {
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
            ExaltStateView::Exalt(exalt) => exalt.set_essence_rating(rating),
            ExaltStateView::Mortal(_) => Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::MortalError,
            )),
        }?;
        Ok(self)
    }
}
