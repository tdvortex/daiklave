use thiserror::Error;

/// An error trying to set an Attribute rating.
#[derive(Debug, Error)]
pub enum SetAttributesError {
    /// Attributes must be between 1 and 5.
    #[error("Attributes must be between 1 and 5, {0} is invalid")]
    InvalidRating(u8),
}
