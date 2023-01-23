use thiserror::Error;

/// An error relating to Merits.
#[derive(Debug, Error)]
pub enum MeritError {
    /// Can't have two merits with the same Id or two flaws with the same name
    #[error("Can't have two merits with the same Id, or two Flaws with the same name")]
    DuplicateMerit,
    /// All Exalts must have Exalted Healing
    #[error("Exalted Healing is required for all Exalts")]
    ExaltedHealing,
    /// Merit templates require at least one valid dot rating.
    #[error("At least one valid rating required")]
    MissingDotRating,
    /// Can't find that merit or flaw
    #[error("Merit/Flaw not found")]
    NotFound,
    /// Merit templates can only define dot ratings from 0 to 5, and instances
    /// can only select from these available ratings.
    #[error("Dot rating must be 0 to 5 and must be allowed by their template")]
    InvalidDotRating,
    /// Can't add a merit if it has prerequisites that are not met.
    #[error("Merit prerequisites have not been met")]
    PrerequisitesNotMet,
}
