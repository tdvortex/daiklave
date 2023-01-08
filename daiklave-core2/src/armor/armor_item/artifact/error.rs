use thiserror::Error;

/// An error related to artifacts.
#[derive(Debug, Error)]
pub enum ArtifactError {
    /// All named artifacts must be unique on a character
    #[error("Named artifacts must be unique")]
    NamedArtifactsUnique,
}
