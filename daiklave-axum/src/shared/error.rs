use serenity::all::ChannelId;
use thiserror::Error;

/// An error related to a (manually enforced) database constraint.
#[derive(Debug, Error)]
pub enum ConstraintError {
    /// Campaigns-to-channels is one-to-many; each channel can only have one
    /// campaign.
    #[error("Channel {0:?} already in use")]
    ChannelCampaignUnique(ChannelId),
}


/// An error that occurs when interacting with the data layer (Redis+MongoDB).
#[derive(Debug, Error)]
pub enum DatabaseError {
    /// A data constraint would be violated by this operation
    #[error("Constraint violated: {0:?}")]
    ConstraintError(#[from] ConstraintError),
    /// An error occurred attempting to deserialize a piece of data
    #[error("Could not deserialize {0} from the database")]
    DeserializationError(String),
    /// An error occurred attempting to serialize a piece of data
    #[error("An error occurred while serializing {0}")]
    SerializationError(String),
    /// MongoDb returned an error
    #[error("An error occurred connecting to MongoDb")]
    MongoDb(#[from] mongodb::error::Error),
}