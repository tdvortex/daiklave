use thiserror::Error;

/// An error related to Intimacies.
#[derive(Debug, Error)]
pub enum IntimacyError {
    /// Only one instance of an IntimacyId per character.
    #[error("Cannot have duplicate Intimacies")]
    DuplicateIntimacy,
    /// Cannot remove or alter a missing Intimacy.
    #[error("Intimacy not found")]
    NotFound,
}