use crate::{
    essence::{
        CommitMotesError, RecoverMotesError, SetEssenceRatingError, SpendMotesError,
        UncommitMotesError,
    },
    exalt_type::ExaltState,
    CharacterMutationError, CommittedMotesId, MotePool,
};

use super::Essence;

impl ExaltState {
    pub(in crate::essence) fn essence(&self) -> Option<&Essence> {
        match self {
            ExaltState::Mortal => None,
            ExaltState::Exalted(exalt_type) => Some(exalt_type.essence()),
        }
    }

    pub(in crate::essence) fn check_spend_motes(
        &self,
        first: MotePool,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal => Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::MortalError,
            )),
            ExaltState::Exalted(exalt_type) => exalt_type.check_spend_motes(first, amount),
        }
    }

    pub(in crate::essence) fn spend_motes(
        &mut self,
        first: MotePool,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal => Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::MortalError,
            )),
            ExaltState::Exalted(exalt_type) => exalt_type.spend_motes(first, amount),
        }?;
        Ok(self)
    }

    pub(in crate::essence) fn check_commit_motes(
        &self,
        id: &CommittedMotesId,
        name: &str,
        first: MotePool,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal => Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::MortalError,
            )),
            ExaltState::Exalted(exalt_type) => {
                exalt_type.check_commit_motes(id, name, first, amount)
            }
        }
    }

    pub(in crate::essence) fn commit_motes(
        &mut self,
        id: &CommittedMotesId,
        name: &str,
        first: MotePool,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal => Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::MortalError,
            )),
            ExaltState::Exalted(exalt_type) => exalt_type.commit_motes(id, name, first, amount),
        }?;
        Ok(self)
    }

    pub(in crate::essence) fn check_recover_motes(
        &self,
        _amount: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            ExaltState::Exalted(_) => Ok(()),
        }
    }

    pub(in crate::essence) fn recover_motes(
        &mut self,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            ExaltState::Exalted(exalt_type) => exalt_type.recover_motes(amount),
        }?;
        Ok(self)
    }

    pub(in crate::essence) fn check_uncommit_motes(
        &self,
        id: &CommittedMotesId,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            ExaltState::Exalted(exalt_type) => exalt_type.check_uncommit_motes(id),
        }
    }

    pub(in crate::essence) fn uncommit_motes(
        &mut self,
        id: &CommittedMotesId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            ExaltState::Exalted(exalt_type) => exalt_type.uncommit_motes(id),
        }?;
        Ok(self)
    }

    pub(in crate::essence) fn check_set_essence_rating(
        &self,
        rating: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal => Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::MortalError,
            )),
            ExaltState::Exalted(_) => {
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

    pub(in crate::essence) fn set_essence_rating(
        &mut self,
        rating: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_essence_rating(rating)?;
        match self {
            ExaltState::Exalted(exalt_type) => exalt_type.set_essence_rating(rating),
            ExaltState::Mortal => Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::MortalError,
            )),
        }?;
        Ok(self)
    }
}
