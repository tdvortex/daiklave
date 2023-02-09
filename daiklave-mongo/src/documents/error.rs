use thiserror::Error;

/// An error related to using a MongoDb document.
#[derive(Debug, Error)]
pub enum DocumentError {
    /// Each channel can only belong to one campaign.
    #[error("Channel is already in use")]
    DuplicateChannelCampaign,
    /// An error in deserializing a MongoDb result.
    #[error("An error occured while deserializing a MongoDb response")]
    DeserializationError,
    /// Wraps an error from MongoDb.
    #[error("MongoDb returned an error")]
    MongoError(#[from] mongodb::error::Error),
    /// Failed to find a document using find_one.
    #[error("Document not found")]
    NotFound,
    /// An error in trying to serialize something into a BSON document
    #[error("An error occurred while serializing into BSON")]
    SerializationError,
}