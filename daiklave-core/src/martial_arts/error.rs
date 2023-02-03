use thiserror::Error;

/// An error related to Martial Arts.
#[derive(Debug, Error)]
pub enum MartialArtsError {
    /// Style already exists.
    #[error("Already have style with this id")]
    DuplicateStyle,
    /// Prerequisite conditions were not met.
    #[error("Prerequisites not met")]
    PrerequsitesNotMet,
    /// Can't remove a missing style
    #[error("Style not found")]
    StyleNotFound,
}
