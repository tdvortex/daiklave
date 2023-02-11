use thiserror::Error;

/// An error that occurs when interacting with the data layer (Redis+MongoDB).
#[derive(Debug, Error)]
pub enum DataError {
    /// An error occurred connecting to MongoDb
    #[error("An error occurred connecting to MongoDb")]
    MongoDb(#[from] mongodb::error::Error),
}