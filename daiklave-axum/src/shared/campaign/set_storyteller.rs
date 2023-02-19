use std::collections::HashSet;

use mongodb::{
    bson::{doc, oid::ObjectId},
    ClientSession,
};
use redis::AsyncCommands;
use serenity::all::{ChannelId, UserId};

use crate::{
    mongo::{campaigns::CampaignCurrent, users::UserCurrent},
    shared::{
        error::{ConstraintError, DatabaseError},
        to_bson,
    },
};

/// An instruction to hand off storyteller permissions to another player in the
/// campaign.
pub struct SetCampaignStoryteller {
    /// The campaign to be modified
    pub campaign_id: ObjectId,
    /// The old storyteller's UserId
    pub old_storyteller: UserId,
    /// The UserId for the new storyteller
    pub new_storyteller: UserId,
}

impl SetCampaignStoryteller {
    async fn execute_mongo(
        &self,
        database: &mongodb::Database,
        session: &mut ClientSession,
    ) -> Result<HashSet<ChannelId>, DatabaseError> {
        session.start_transaction(None).await?;
        // Get the campaign document
        let campaigns = database.collection::<CampaignCurrent>("campaigns");
        let filter = doc! {
            "_id": &self.campaign_id,
        };
        let campaign = campaigns
            .find_one_with_session(filter, None, session)
            .await?
            .ok_or_else(|| DatabaseError::NotFound("Campaign".to_owned()))?;

        // Validate new storyteller is a player
        if !campaign.players.contains(&self.new_storyteller) {
            return Err(DatabaseError::ConstraintError(
                ConstraintError::StorytellerNotPlayer,
            ));
        }

        // Replace the campaign document
        let mut new_campaign = campaign.clone();
        new_campaign.storyteller = self.new_storyteller;
        let query = doc! {
            "_id": &self.campaign_id,
        };
        campaigns
            .replace_one_with_session(query, new_campaign, None, session)
            .await?;

        // Update the old storyteller's user document
        let users = database.collection::<UserCurrent>("users");
        let query = doc! {
            "discordId": to_bson(&self.old_storyteller)?,
            "campaigns": {
                "campaignId": &self.campaign_id
            }
        };
        let update = doc! {
            "$set": {
                "campaigns.$.isStoryteller": false,
            }
        };
        users
            .update_one_with_session(query, update, None, session)
            .await?;

        // Update the new storyteller's user document
        let query = doc! {
            "discordId": to_bson(&self.new_storyteller)?,
            "campaigns": {
                "campaignId": &self.campaign_id
            }
        };
        let update = doc! {
            "$set": {
                "campaigns.$.isStoryteller": true,
            }
        };
        users
            .update_one_with_session(query, update, None, session)
            .await?;

        // Return all of the channels for the campaign for Redis updating
        Ok(campaign.channels.clone())
    }

    async fn execute_redis<CON: AsyncCommands>(
        &self,
        channels: HashSet<ChannelId>,
        connection: &mut CON,
    ) -> Result<(), DatabaseError> {
        // Delete all of the campaign and channel fields in the old storyteller's Redis hash
        // New permissions will be lazy-loaded as needed
        let mut key = "userId:".as_bytes().to_vec();
        key.extend(self.old_storyteller.0.get().to_be_bytes());

        let mut fields = Vec::new();
        let mut campaign_field = "campaignId:".as_bytes().to_vec();
        campaign_field.extend(self.campaign_id.bytes());
        fields.push(campaign_field);

        for channel in channels.into_iter() {
            let mut channel_field = "channelId:".as_bytes().to_vec();
            channel_field.extend(channel.0.get().to_be_bytes());
            fields.push(channel_field);
        }

        connection.hdel(key, fields).await?;

        Ok(())
    }

    /// Updates the storyteller for the campaign. Requires a session to update
    /// the campaign and player collections atomically.
    pub async fn execute<CON: AsyncCommands>(
        &self,
        database: &mongodb::Database,
        session: &mut ClientSession,
        connection: &mut CON,
    ) -> Result<(), DatabaseError> {
        let channels = self.execute_mongo(database, session).await?;
        self.execute_redis(channels, connection).await?;
        Ok(())
    }
}
