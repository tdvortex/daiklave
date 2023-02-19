use mongodb::bson::{doc};
use serenity::all::{UserId, ChannelId};

use crate::{
    mongo::{
        campaigns::CampaignCurrent,
        users::{InsertUser, PlayerCampaign, UserCurrent, UserVersion}, channels::ChannelCurrent,
    },
    shared::{error::DatabaseError, to_bson},
};

/// An instruction to add a player to a campaign.
pub struct AddCampaignPlayer {
    /// The channel Id the player is joining from.
    pub channel_id: ChannelId,
    /// The Discord Id of the user to add.
    pub user_id: UserId,
}

impl AddCampaignPlayer {
    /// Adds the player to the campaign. Requires a session to atomically
    /// update the campaign document and the player document. Does not use
    /// Redis; permission updates will be lazy-loaded when used.
    pub async fn execute(
        &self,
        database: &mongodb::Database,
        session: &mut mongodb::ClientSession,
    ) -> Result<String, DatabaseError> {
        session.start_transaction(None).await?;

        let channels = database.collection::<ChannelCurrent>("channels");
        let filter = doc! {
            "channelId": to_bson(&self.channel_id)?,
        };
        let channel = channels.find_one_with_session(filter, None, session).await?.ok_or_else(|| DatabaseError::NotFound("Channel".to_owned()))?;
        let campaign_id = channel.campaign_id;

        let campaigns = database.collection::<CampaignCurrent>("campaigns");
        let filter = doc! {
            "_id": &campaign_id,
        };
        let campaign = campaigns
            .find_one_with_session(filter, None, session)
            .await?
            .ok_or_else(|| DatabaseError::NotFound("Campaign".to_owned()))?;

        if campaign.players.contains(&self.user_id) {
            // Nothing to do
            session.commit_transaction().await?;
            return Ok(campaign.name);
        }

        let users = database.collection::<UserCurrent>("users");
        let filter = doc! {
            "discordId": to_bson(&self.user_id)?,
        };
        let maybe_player = users.find_one_with_session(filter, None, session).await?;

        let player_campaign = PlayerCampaign {
            campaign_id: campaign._id,
            name: campaign.name.clone(),
            is_storyteller: false,
            characters: Default::default(),
            dice_channel: campaign.dice_channel,
            channels: campaign.channels,
        };

        if let Some(user) = maybe_player {
            let mut new_user = user.clone();
            new_user.campaigns.push(player_campaign);
            let query = doc! {
                "discordId": to_bson(&self.user_id)?,
            };
            users
                .replace_one_with_session(query, new_user, None, session)
                .await?;
        } else {
            let users = database.collection::<InsertUser>("users");
            users
                .insert_one_with_session(
                    InsertUser {
                        version: UserVersion::V0,
                        discord_id: self.user_id,
                        campaigns: vec![player_campaign],
                    },
                    None,
                    session,
                )
                .await?;
        }

        session.commit_transaction().await?;
        Ok(campaign.name)
    }
}
