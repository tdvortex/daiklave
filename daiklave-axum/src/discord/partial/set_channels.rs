use std::collections::HashSet;

use mongodb::bson::oid::ObjectId;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serenity::all::ChannelId;

use crate::shared::error::DatabaseError;

/// An in-progress update to a campaign's channels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialSetChannels {
    /// The Id of the campaign being updated
    pub campaign_id: ObjectId,
    /// If set, the dice channel for the campaign
    pub dice_channel: Option<ChannelId>,
    /// If set, the other channels for the campaign
    pub channels: HashSet<ChannelId>,
}

impl PartialSetChannels {
    /// Saves a partially-loaded channels update to Redis with the interaction
    /// token as the key.
    pub async fn save_partial<CON: AsyncCommands>(
        &self,
        token: String,
        connection: &mut CON,
    ) -> Result<(), DatabaseError> {
        let partial_bytes = postcard::to_allocvec(&self)
            .map_err(|_| DatabaseError::SerializationError(format!("{:?}", self)))?;

        connection.set_ex(token, partial_bytes, 1000).await?;

        Ok(())
    }

    /// Loads a partially-loaded channels update from Redis with the interaction
    /// token as the key.
    pub async fn load_partial<CON: AsyncCommands>(
        token: String,
        connection: &mut CON,
    ) -> Result<Option<Self>, DatabaseError> {
        let maybe_partial_bytes: Option<Vec<u8>> = connection.get(token).await?;

        if let Some(bytes) = maybe_partial_bytes {
            Ok(Some(postcard::from_bytes(&bytes).map_err(|_| {
                DatabaseError::DeserializationError("PartialSetChannels".to_owned())
            })?))
        } else {
            Ok(None)
        }
    }
}
