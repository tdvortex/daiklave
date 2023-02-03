use thiserror::Error;

/// An error occurring while attempting to set or remove a character's concept.
#[derive(Debug, Error)]
pub enum ConceptError {
    /// Can't remove a missing character concept
    #[error("Character does not have a concept to remove")]
    NotFound,
}
