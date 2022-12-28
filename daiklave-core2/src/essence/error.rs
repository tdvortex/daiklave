use thiserror::Error;

use crate::CommittedMotesId;

#[derive(Debug, Error)]
pub enum CommitMotesError {
    #[error("Mortals do not have Essence")]
    MortalError,
    #[error("Insufficient motes, need {1} but only have {0}")]
    InsufficientMotes(u8, u8),
}

#[derive(Debug, Error)]
pub enum RecoverMotesError {
    #[error("Mortals do not have Essence")]
    MortalError,
}

#[derive(Debug, Error)]
pub enum UncommitMotesError {
    #[error("Mortals do not have Essence")]
    MortalError,
    #[error("Mote commitment id {0:?} not found")]
    NotFound(CommittedMotesId),
}

#[derive(Debug, Error)]
pub enum SetEssenceRatingError {
    #[error("Mortals do not have Essence")]
    MortalError,
    #[error("Essence must be between 1 and 5, not {0}")]
    InvalidRating(u8),
}

#[derive(Debug, Error)]
pub enum SpendMotesError {
    #[error("Mortals do not have Essence")]
    MortalError,
    #[error("Insufficient motes, need {1} but only have {0}")]
    InsufficientMotes(u8, u8),
}
