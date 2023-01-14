use thiserror::Error;

/// An error related to Attributes.
#[derive(Debug, Error)]
pub enum AttributeError {
    /// Attributes must be between 1 and 5.
    #[error("Attributes must be between 1 and 5")]
    InvalidRating,
}
