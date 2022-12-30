use crate::{exalt_state::ExaltStateView, CharacterMutationError};

use super::{
    CommitMotesError, CommittedMotesId, EssenceView, MotePool, RecoverMotesError,
    SetEssenceRatingError, SpendMotesError, UncommitMotesError,
};

impl<'source> ExaltStateView<'source> {
    pub(in crate::exalt_state::exalt::essence) fn essence(&self) -> Option<&EssenceView> {
        match self {
            ExaltStateView::Mortal(_) => None,
            ExaltStateView::Exalt(exalt) => Some(exalt.essence()),
        }
    }

    pub(in crate::exalt_state::exalt::essence) fn check_spend_motes(
        &self,
        first: MotePool,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(_) => Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::MortalError,
            )),
            ExaltStateView::Exalt(exalt) => exalt.check_spend_motes(first, amount),
        }
    }

    pub(in crate::exalt_state::exalt::essence) fn spend_motes(
        &mut self,
        first: MotePool,
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

    pub(in crate::exalt_state::exalt::essence) fn check_commit_motes(
        &self,
        id: &CommittedMotesId,
        name: &str,
        first: MotePool,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(_) => Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::MortalError,
            )),
            ExaltStateView::Exalt(exalt) => exalt.check_commit_motes(id, name, first, amount),
        }
    }

    pub(in crate::exalt_state::exalt::essence) fn commit_motes(
        &mut self,
        id: &CommittedMotesId,
        name: &'source str,
        first: MotePool,
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

    pub(in crate::exalt_state::exalt::essence) fn check_recover_motes(
        &self,
        _amount: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(_) => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            ExaltStateView::Exalt(_) => Ok(()),
        }
    }

    pub(in crate::exalt_state::exalt::essence) fn recover_motes(
        &mut self,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(_) => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            ExaltStateView::Exalt(exalt) => exalt.recover_motes(amount),
        }?;
        Ok(self)
    }

    pub(in crate::exalt_state::exalt::essence) fn check_uncommit_motes(
        &self,
        id: &CommittedMotesId,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(_) => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            ExaltStateView::Exalt(exalt) => exalt.check_uncommit_motes(id),
        }
    }

    pub(in crate::exalt_state::exalt::essence) fn uncommit_motes(
        &mut self,
        id: &CommittedMotesId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(_) => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            ExaltStateView::Exalt(exalt) => exalt.uncommit_motes(id),
        }?;
        Ok(self)
    }

    pub(in crate::exalt_state::exalt::essence) fn check_set_essence_rating(
        &self,
        rating: u8,
    ) -> Result<(), CharacterMutationError> {
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

    pub(in crate::exalt_state::exalt::essence) fn set_essence_rating(
        &mut self,
        rating: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
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
