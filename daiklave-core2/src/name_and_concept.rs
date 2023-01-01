use thiserror::Error;

/// An error occurring while attempting to set or remove a character's concept.
#[derive(Debug, Error)]
pub enum RemoveConceptError {
    #[error("character does not have a concept")]
    NoConcept,
}
