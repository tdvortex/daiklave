use thiserror::Error;

/// An error when tryng to add a Martial Arts style to a character.
#[derive(Debug, Error)]
pub enum AddMartialArtsStyleError {
    /// Prerequisite conditions were not met.
    #[error("Prerequisite not met: {0}")]
    PrerequsitesNotMet(String),
    /// Style already exists.
    #[error("Already have style with this id")]
    DuplicateStyle,
}

/// An error when tryng to remove a Martial Arts style from a character.
#[derive(Debug, Error)]
pub enum RemoveMartialArtsStyleError {
    /// Can't remove a missing style
    #[error("Style not found")]
    NotFound,
}

/// An error when trying to set Martial Arts dots.
#[derive(Debug, Error)]
pub enum SetMartialArtsDotsError {
    /// Can't change dots on a missing style
    #[error("Style not found")]
    NotFound,
}
