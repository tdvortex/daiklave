use thiserror::Error;

/// An error that occurs when interacting with the data layer (Redis+MongoDB).
#[derive(Debug, Error)]
pub enum DataError {
    /// An error occurred attempting to serialize a piece of data
    #[error("An error occurred while serializing {0}")]
    SerializationError(String),
    /// MongoDb returned an error
    #[error("An error occurred connecting to MongoDb")]
    MongoDb(#[from] mongodb::error::Error),
}