use thiserror::Error;

use super::CommittedMotesId;

/// An error when trying to commit motes to an ongoing effect.
#[derive(Debug, Error)]
pub enum CommitMotesError {
    /// Mortals cannot commit motes.
    #[error("Mortals do not have Essence")]
    MortalError,
    /// Can't commit more motes than available.
    #[error("Insufficient motes, need {1} but only have {0}")]
    InsufficientMotes(u8, u8),
}

/// An error when trying to recover spent motes.
#[derive(Debug, Error)]
pub enum RecoverMotesError {
    /// Mortals cannot recover motes.
    #[error("Mortals do not have Essence")]
    MortalError,
}

/// An error when trying to uncommit motes from an ongoing effect.
#[derive(Debug, Error)]
pub enum UncommitMotesError {
    /// Mortals cannot uncommit motes.
    #[error("Mortals do not have Essence")]
    MortalError,
    /// Cannot uncommit an effect that does not exist.
    #[error("Mote commitment id {0:?} not found")]
    NotFound(CommittedMotesId),
}

/// An error when trying to set the essence rating of a character.
#[derive(Debug, Error)]
pub enum SetEssenceRatingError {
    /// Mortals do not have an Essence rating (or, it's always 1).
    #[error("Mortals do not have Essence")]
    MortalError,
    /// Essence ratings can only be between 1 and 5.
    #[error("Essence must be between 1 and 5, not {0}")]
    InvalidRating(u8),
}

/// An error when trying to spend motes.
#[derive(Debug, Error)]
pub enum SpendMotesError {
    /// Mortals cannot spend motes.
    #[error("Mortals do not have Essence")]
    MortalError,
    /// Cannot spend more motes than you have (peripheral + personal combined).
    #[error("Insufficient motes, need {1} but only have {0}")]
    InsufficientMotes(u8, u8),
}
