use std::collections::HashSet;

use mongodb::{
    bson::{self, doc, oid::ObjectId},
    ClientSession,
};
use serenity::all::{ChannelId, UserId};

use crate::{
    mongo::{campaigns::CampaignCurrent, characters::CharacterCurrent, users::UserCurrent},
    shared::error::{ConstraintError, DatabaseError},
};

/// Removes a player from a campaign. The storyteller cannot be removed.
pub struct RemoveCampaignPlayer {
    /// The Id of the campaign.
    pub campaign_id: ObjectId,
    /// The Id of the user to add.
    pub user_id: UserId,
}

impl RemoveCampaignPlayer {
    async fn execute_mongo(
        &self,
        database: &mongodb::Database,
        session: &mut ClientSession,
    ) -> Result<HashSet<ChannelId>, DatabaseError> {
        session.start_transaction(None).await?;

        let player_bson = bson::to_bson(&self.user_id)
            .map_err(|_| DatabaseError::SerializationError("user_id".to_owned()))?;

        // Get the campaign document for this campaign
        let campaigns = database.collection::<CampaignCurrent>("campaigns");
        let filter = doc! {
            "_id": self.campaign_id
        };
        let mut campaign = campaigns
            .find_one_with_session(filter, None, session)
            .await?
            .ok_or_else(|| DatabaseError::NotFound(format!("campaign_id: {}", self.campaign_id)))?;

        if campaign.storyteller == self.user_id {
            return Err(DatabaseError::ConstraintError(
                ConstraintError::RemoveStoryteller,
            ));
        }

        // Update the campaign document to no longer have this player
        if !campaign.players.remove(&self.user_id) {
            // Nothing to do
            session.commit_transaction().await?;
            return Ok(HashSet::new());
        }
        let campaign_channels = campaign.channels.clone();
        let query = doc! {
            "_id": self.campaign_id
        };
        campaigns
            .replace_one_with_session(query, campaign, None, session)
            .await?;

        // Remove the campaign from the player
        let users = database.collection::<UserCurrent>("users");
        let query = doc! {
            "discordId": &player_bson
        };
        let update = doc! {
            "$pull": {
                "campaigns": {
                    "campaignId": self.campaign_id
                }
            }
        };
        users
            .update_one_with_session(query, update, None, session)
            .await?;

        // Remove all characters this player has with this campaign
        let characters = database.collection::<CharacterCurrent>("characters");
        let query = doc! {
            "player": &player_bson,
            "campaignId": self.campaign_id
        };
        characters
            .delete_many_with_session(query, None, session)
            .await?;

        // Done with database
        session.commit_transaction().await?;

        // Return the channels to invalidate
        Ok(campaign_channels)
    }

    async fn execute_redis<CON>(
        &self,
        connection: &mut CON,
        campaign_channels: HashSet<ChannelId>,
    ) -> Result<(), redis::RedisError>
    where
        CON: redis::AsyncCommands,
    {
        let mut hdel_command = redis::cmd("HDEL");

        let mut key = "userId:".as_bytes().to_vec();
        key.extend(self.user_id.0.get().to_be_bytes());
        hdel_command.arg(key);

        let mut campaign_field = "campaignId:".as_bytes().to_vec();
        campaign_field.extend(self.campaign_id.bytes());
        hdel_command.arg(campaign_field);

        for channel in campaign_channels {
            let mut channel_field = "channelId:".as_bytes().to_vec();
            channel_field.extend(channel.0.get().to_be_bytes());
            hdel_command.arg(channel_field);
        }

        hdel_command.query_async(connection).await?;

        Ok(())
    }

    /// Remove a player from a campaign (either leaving or kicking).
    pub async fn execute<CON>(
        &self,
        database: &mongodb::Database,
        session: &mut ClientSession,
        connection: &mut CON,
    ) -> Result<(), DatabaseError>
    where
        CON: redis::AsyncCommands,
    {
        // Update database
        let channels = self.execute_mongo(database, session).await?;
        // Invalidate cache
        self.execute_redis(connection, channels).await?;
        Ok(())
    }
}
