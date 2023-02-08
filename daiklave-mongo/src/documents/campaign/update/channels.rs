use std::collections::HashSet;

use bson::{doc, oid::ObjectId, Bson};
use futures::TryStreamExt;
use serenity::all::{ChannelId, UserId};

use crate::{
    campaign::{CampaignCurrent, CampaignDocument},
    channel::{ChannelCurrent, ChannelVersion, CreateChannel},
    error::DocumentError,
    user::UserDocument,
};

/// Instruction to update the channels for a campaign.
pub struct UpdateCampaignChannels {
    /// The MongoDB database ID for the campaign.
    pub _id: ObjectId,
    /// The Id of the channel to which dice rolls are sent when invoked from
    /// the browser. (Slash commands will roll dice in the channel where they
    /// are invoked.)
    pub dice_channel: ChannelId,
    /// All channels that the campaign claims ownership of (including the dice
    /// channel)
    pub channels: HashSet<ChannelId>,
}

impl UpdateCampaignChannels {
    /// Updates the channel list for a campaign. This requires a session to
    /// update campaigns, users, and channels atomically. Returns a vector of
    /// pairs of (channel_id, player_id) permissions which must be invalidated
    /// in the cache.
    pub async fn execute(
        &self,
        database: &mongodb::Database,
        session: &mut mongodb::ClientSession,
    ) -> Result<Vec<(ChannelId, UserId)>, DocumentError> {
        session.start_transaction(None).await?;

        // Get the existing campaign document, extract its current players and channels
        let campaigns = database.collection::<CampaignCurrent>("campaigns");
        let filter = doc! {
            "_id": self._id
        };
        let campaign = campaigns
            .find_one_with_session(filter, None, session)
            .await?
            .ok_or(DocumentError::NotFound)?;
        let campaign_players = campaign.players.clone();
        let old_channels = campaign.channels.clone();
        let new_channels = self.channels.clone();

        // Check to make sure none of the channels are in use by a diffent campaign
        let channels = database.collection::<ChannelCurrent>("channels");
        let channel_ids: Vec<Bson> = old_channels
            .iter()
            .map(|channel_id| bson::to_bson(channel_id).or(Err(DocumentError::SerializationError)))
            .fold(
                Ok(Vec::new()),
                |acc: Result<Vec<Bson>, DocumentError>, res_bson| match (acc, res_bson) {
                    (Ok(mut v), Ok(b)) => {
                        v.push(b);
                        Ok(v)
                    }
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                },
            )?;
        let filter = doc! {
            "channelId": {
                "$in": channel_ids
            }
        };
        let any_channels_in_use = channels
            .find_with_session(filter, None, session)
            .await?
            .stream(session)
            .try_collect::<Vec<ChannelCurrent>>()
            .await?
            .into_iter()
            .any(|current| current.campaign_id != self._id);
        if any_channels_in_use {
            return Err(DocumentError::DuplicateChannelCampaign);
        }

        // Identify channels to add and drop
        let drop_channels = old_channels
            .difference(&new_channels)
            .copied()
            .collect::<Vec<ChannelId>>();
        let add_channels = new_channels
            .difference(&old_channels)
            .copied()
            .collect::<Vec<ChannelId>>();

        // Drop the removed channels
        let drop_channels_bson = drop_channels
            .iter()
            .map(|channel_id| bson::to_bson(channel_id).or(Err(DocumentError::SerializationError)))
            .fold(
                Ok(Vec::new()),
                |acc: Result<Vec<Bson>, DocumentError>, res_bson| match (acc, res_bson) {
                    (Ok(mut v), Ok(b)) => {
                        v.push(b);
                        Ok(v)
                    }
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                },
            )?;
        let query = doc! {
            "channelId": {
                "$in": drop_channels_bson
            }
        };
        channels
            .delete_many_with_session(query, None, session)
            .await?;

        // Instantiate new channel documents for this campaign
        let new_channels = add_channels
            .into_iter()
            .map(|channel_id| CreateChannel {
                version: ChannelVersion::V0,
                channel_id,
                campaign_id: self._id,
            })
            .collect::<Vec<CreateChannel>>();
        let channels = database.collection::<CreateChannel>("channels");
        channels
            .insert_many_with_session(new_channels, None, session)
            .await?;

        // Override the channels in the campaign document
        let campaigns = database.collection::<CampaignDocument>("campaigns");
        let query = doc! {
            "_id": self._id
        };
        let update = doc! {
            "$set": {
                "diceChannel": bson::to_bson(&self.dice_channel).or(Err(DocumentError::SerializationError))?,
                "channels": bson::to_bson(&self.channels).or(Err(DocumentError::SerializationError))?,
            }
        };
        campaigns
            .update_one_with_session(query, update, None, session)
            .await?;

        // Override the channels in all relevant player documents
        let users = database.collection::<UserDocument>("users");
        let query = doc! {
            "campaigns": {
                "campaignId": self._id
            }
        };
        let update = doc! {
            "$set": {
                "campaigns.$[elem].diceChannel" : bson::to_bson(&self.dice_channel).or(Err(DocumentError::SerializationError))?,
                "campaigns.$[elem].channels" : bson::to_bson(&self.dice_channel).or(Err(DocumentError::SerializationError))?,
            }
        };
        let array_filter = doc! {
            "elem.campaignId": self._id
        };
        let options = mongodb::options::UpdateOptions::builder()
            .array_filters(Some(vec![array_filter]))
            .build();
        users
            .update_many_with_session(query, update, options, session)
            .await?;

        // Done with the database
        session.commit_transaction().await?;

        // Build the set of channel-player pairs that is no longer valid for this campaign
        Ok(drop_channels
            .into_iter()
            .flat_map(|channel_id| {
                campaign_players
                    .iter()
                    .map(move |&discord_id| (channel_id, discord_id))
            })
            .collect::<Vec<(ChannelId, UserId)>>())
    }
}
