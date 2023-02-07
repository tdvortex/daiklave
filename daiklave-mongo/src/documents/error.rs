use thiserror::Error;

/// An error related to using a MongoDb document.
#[derive(Debug, Error)]
pub enum DocumentError {
    /// Wraps an error from MongoDb.
    #[error("MongoDb returned an error")]
    MongoError(#[from] mongodb::error::Error),
    /// An error in deserializing a MongoDb result.
    #[error("An error occured while deserializing a MongoDb response")]
    DeserializationError,
    /// An error in trying to serialize something into a BSON document
    #[error("An error occurred while serializing into BSON")]
    SerializationError,
}