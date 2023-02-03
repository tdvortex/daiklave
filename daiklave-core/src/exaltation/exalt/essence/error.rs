use thiserror::Error;

/// An error related to motes or Essence ratings.
#[derive(Debug, Error)]
pub enum EssenceError {
    /// Can't attune to the same artifact twice.
    #[error("Already attuned to that artifact")]
    AlreadyAttuned,
    /// Can't commit a duplicate mote commitment
    #[error("Mote commitment Ids must be unique")]
    DuplicateCommitment,
    /// Can't spend or commit more motes than you have
    #[error("Insufficient motes")]
    InsufficientMotes,
    /// Essence ratings can only be between 1 and 5.
    #[error("Essence must be between 1 and 5")]
    InvalidRating,
    /// Mortals cannot do anything with essence..
    #[error("Mortals do not have Essence")]
    Mortal,
    /// Can't attune to something if it doesn't have an attunement cost
    #[error("No attunement cost, cannot attune")]
    NoAttunementCost,
    /// Not all Exalted have Limit
    #[error("Character does not have Limit")]
    NoLimit,
    /// Mote commitment does not exist
    #[error("Mote commitment not found")]
    NotFound,
}
