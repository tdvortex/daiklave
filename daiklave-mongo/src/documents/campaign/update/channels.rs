use std::collections::HashSet;

use bson::{doc, oid::ObjectId, Bson};
use futures::TryStreamExt;
use serenity::all::ChannelId;

use crate::{
    campaign::CampaignDocument,
    channel::{ChannelCurrent, ChannelDocument, ChannelVersion, NewChannel},
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
    /// update campaigns, users, and channels atomically.
    pub async fn update(
        &self,
        database: &mongodb::Database,
        session: &mut mongodb::ClientSession,
    ) -> Result<(), DocumentError> {
        session.start_transaction(None).await?;

        // Check to make sure none of the channels are in use by a diffent campaign
        let channels = database.collection::<ChannelDocument>("channels");
        let channel_ids: Vec<Bson> = self
            .channels
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
            .try_collect::<Vec<ChannelDocument>>()
            .await?
            .into_iter()
            .map(|document| ChannelCurrent::from(document))
            .any(|current| current.campaign_id != self._id);
        if any_channels_in_use {
            return Err(DocumentError::DuplicateChannelCampaign);
        }

        // Drop all existing channel documents for this campaign
        let query = doc! {
            "campaignId": self._id
        };
        channels
            .delete_many_with_session(query, None, session)
            .await?;

        // Instantiate new channel documents for this campaign
        let mut new_channels = Vec::new();

        for channel in self.channels.iter() {
            let new_channel = NewChannel {
                version: ChannelVersion::V0,
                channel_id: *channel,
                campaign_id: self._id,
            };
            new_channels.push(new_channel);
        }
        let channels = database.collection::<NewChannel>("channels");
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

        session.commit_transaction().await?;
        Ok(())
    }
}
