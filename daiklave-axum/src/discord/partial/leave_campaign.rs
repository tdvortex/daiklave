use mongodb::bson::oid::ObjectId;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serenity::all::UserId;

use crate::shared::error::DatabaseError;

/// An instruction for a player to leave a campaign.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialLeaveCampaign {
    /// The Id of the campaign the player is leaving
    pub campaign_id: ObjectId,
    /// The Id of the player leaving the campaign.
    pub user_id: UserId,
}

impl PartialLeaveCampaign {
    /// Saves the command data to Redis.
    pub async fn save<CON: AsyncCommands>(
        &self,
        token: String,
        connection: &mut CON,
    ) -> Result<(), DatabaseError> {
        let partial_bytes = postcard::to_allocvec(&self)
            .map_err(|_| DatabaseError::SerializationError(format!("{:?}", self)))?;

        connection.set_ex(token, partial_bytes, 1000).await?;

        Ok(())
    }

    /// Loads the command data from Redis.
    pub async fn load<CON: AsyncCommands>(
        token: String,
        connection: &mut CON,
    ) -> Result<Option<Self>, DatabaseError> {
        let maybe_partial_bytes: Option<Vec<u8>> = connection.get(token).await?;

        if let Some(bytes) = maybe_partial_bytes {
            Ok(Some(postcard::from_bytes(&bytes).map_err(|_| {
                DatabaseError::DeserializationError("PartialLeaveCampaign".to_owned())
            })?))
        } else {
            Ok(None)
        }
    }
}
