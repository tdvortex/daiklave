use crate::{
    essence::{
        CommitMotesError, RecoverMotesError, SetEssenceRatingError, SpendMotesError,
        UncommitMotesError,
    },
    exalt_type::ExaltStateView,
    CharacterMutationError, CommittedMotesId, MotePool,
};

use super::EssenceView;

impl<'source> ExaltStateView<'source> {
    pub(in crate::essence) fn essence(&self) -> Option<&EssenceView> {
        match self {
            ExaltStateView::Mortal => None,
            ExaltStateView::Exalted(exalt_type) => Some(exalt_type.essence()),
        }
    }

    pub(in crate::essence) fn check_spend_motes(
        &self,
        first: MotePool,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltStateView::Mortal => Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::MortalError,
            )),
            ExaltStateView::Exalted(exalt_type) => exalt_type.check_spend_motes(first, amount),
        }
    }

    pub(in crate::essence) fn spend_motes(
        &mut self,
        first: MotePool,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltStateView::Mortal => Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::MortalError,
            )),
            ExaltStateView::Exalted(exalt_type) => exalt_type.spend_motes(first, amount),
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
            ExaltStateView::Mortal => Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::MortalError,
            )),
            ExaltStateView::Exalted(exalt_type) => {
                exalt_type.check_commit_motes(id, name, first, amount)
            }
        }
    }

    pub(in crate::essence) fn commit_motes(
        &mut self,
        id: &CommittedMotesId,
        name: &'source str,
        first: MotePool,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltStateView::Mortal => Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::MortalError,
            )),
            ExaltStateView::Exalted(exalt_type) => exalt_type.commit_motes(id, name, first, amount),
        }?;
        Ok(self)
    }

    pub(in crate::essence) fn check_recover_motes(
        &self,
        _amount: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltStateView::Mortal => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            ExaltStateView::Exalted(_) => Ok(()),
        }
    }

    pub(in crate::essence) fn recover_motes(
        &mut self,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltStateView::Mortal => Err(CharacterMutationError::RecoverMotesError(
                RecoverMotesError::MortalError,
            )),
            ExaltStateView::Exalted(exalt_type) => exalt_type.recover_motes(amount),
        }?;
        Ok(self)
    }

    pub(in crate::essence) fn check_uncommit_motes(
        &self,
        id: &CommittedMotesId,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltStateView::Mortal => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            ExaltStateView::Exalted(exalt_type) => exalt_type.check_uncommit_motes(id),
        }
    }

    pub(in crate::essence) fn uncommit_motes(
        &mut self,
        id: &CommittedMotesId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltStateView::Mortal => Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::MortalError,
            )),
            ExaltStateView::Exalted(exalt_type) => exalt_type.uncommit_motes(id),
        }?;
        Ok(self)
    }

    pub(in crate::essence) fn check_set_essence_rating(
        &self,
        rating: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltStateView::Mortal => Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::MortalError,
            )),
            ExaltStateView::Exalted(_) => {
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
            ExaltStateView::Exalted(exalt_type) => exalt_type.set_essence_rating(rating),
            ExaltStateView::Mortal => Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::MortalError,
            )),
        }?;
        Ok(self)
    }
}
