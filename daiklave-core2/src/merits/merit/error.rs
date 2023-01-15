use thiserror::Error;

/// An error relating to Merits.
#[derive(Debug, Error)]
pub enum MeritError {
    /// Merit templates require at least one valid dot rating.
    #[error("At least one valid rating required")]
    MissingDotRating,
    /// Merit templates can only define dot ratings from 0 to 5, and instances
    /// can only select from these available ratings.
    #[error("Dot rating must be 0 to 5 and must be allowed by their template")]
    InvalidDotRating,
}