use crate::exalt_state::exalt::essence::{MotePool, SpendMotesError, CommittedMotesId, CommitMotesError, RecoverMotesError, UncommitMotesError, SetEssenceRatingError};
use crate::{
    CharacterMutationError,
};

use crate::exalt_state::ExaltState;

use super::Essence;

impl ExaltState {
    pub(in crate::exalt_state::exalt::essence) fn essence(&self) -> Option<&Essence> {
        match self {
            ExaltState::Mortal(_) => None,
            ExaltState::Exalt(exalt) => Some(exalt.essence()),
        }
    }

    pub(in crate::exalt_state::exalt::essence) fn check_spend_motes(
        &self,
        first: MotePool,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal(_) => Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::MortalError,
            )),
            ExaltState::Exalt(exalt) => exalt.check_spend_motes(first, amount),
        }
    }

    pub(in crate::exalt_state::exalt::essence) fn spend_motes(
        &mut self,
        first: MotePool,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal(_) => Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::MortalError,
            )),
            ExaltState::Exalt(exalt) => exalt.spend_motes(first, amount),
        }?;
        Ok(self)
    }

    pub(in crate::exalt_state::exalt::essence) fn check_commit_motes(
        &self,
        id: &CommittedMotesId,
        name: &str,
        first: MotePool,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal(_) => Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::MortalError,
            )),
            ExaltState::Exalt(exalt) => {
                exalt.check_commit_motes(id, name, first, amount)
            }
        }
    }

    pub(in crate::exalt_state::exalt::essence) fn commit_motes(
        &mut self,
        id: &CommittedMotesId,
        name: &str,
        first: MotePool,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal(_) => Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::MortalError,
            )),
            ExaltState::Exalt(exalt) => exalt.commit_motes(id, name, first, amount),
        }?;
        Ok(self)
    }

    pub(in crate::exalt_state::exalt::essence) fn check_recover_motes(
        &self,
        _amount: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal(_) => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            ExaltState::Exalt(_) => Ok(()),
        }
    }

    pub(in crate::exalt_state::exalt::essence) fn recover_motes(
        &mut self,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal(_) => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            ExaltState::Exalt(exalt_type) => exalt_type.recover_motes(amount),
        }?;
        Ok(self)
    }

    pub(in crate::exalt_state::exalt::essence) fn check_uncommit_motes(
        &self,
        id: &CommittedMotesId,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal(_) => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            ExaltState::Exalt(exalt_type) => exalt_type.check_uncommit_motes(id),
        }
    }

    pub(in crate::exalt_state::exalt::essence) fn uncommit_motes(
        &mut self,
        id: &CommittedMotesId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal(_) => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            ExaltState::Exalt(exalt_type) => exalt_type.uncommit_motes(id),
        }?;
        Ok(self)
    }

    pub(in crate::exalt_state::exalt::essence) fn check_set_essence_rating(
        &self,
        rating: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal(_) => Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::MortalError,
            )),
            ExaltState::Exalt(_) => {
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

    pub(in crate::exalt_state::exalt::essence) fn set_essence_rating(
        &mut self,
        rating: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_essence_rating(rating)?;
        match self {
            ExaltState::Exalt(exalt_type) => exalt_type.set_essence_rating(rating),
            ExaltState::Mortal(_) => Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::MortalError,
            )),
        }?;
        Ok(self)
    }
}
