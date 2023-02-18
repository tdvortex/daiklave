use std::collections::HashSet;

use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, UserId};

use crate::shared::error::DatabaseError;

/// A partially-created campaign.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialCreateCampaign {
    /// The name of the campaign (supplied through slash command params)
    pub name: String,
    /// The storyteller of the campaign (the initial command executor)
    pub storyteller: UserId,
    /// If set, the dice channel for the campaign
    pub dice_channel: Option<ChannelId>,
    /// If set, the other channels for the campaign
    pub channels: HashSet<ChannelId>,
}

impl PartialCreateCampaign {
    /// Saves a partially-loaded campaign into Redis with the interaction
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

    /// Loads a partially-loaded campaign into Redis with the interaction
    /// token as the key.
    pub async fn load_partial<CON: AsyncCommands>(
        token: String,
        connection: &mut CON,
    ) -> Result<Option<Self>, DatabaseError> {
        let maybe_partial_bytes: Option<Vec<u8>> = connection.get(token).await?;

        if let Some(bytes) = maybe_partial_bytes {
            Ok(Some(postcard::from_bytes(&bytes).map_err(|_| {
                DatabaseError::DeserializationError("PartialCreateCampaign".to_owned())
            })?))
        } else {
            Ok(None)
        }
    }
}
