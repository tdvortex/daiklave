use std::collections::HashSet;

use mongodb::bson::{oid::ObjectId, Bson, self, doc};
use serde::Deserialize;
use serenity::all::{ChannelId, UserId};

use crate::{shared::error::{DatabaseError, ConstraintError}, mongo::{campaigns::{InsertCampaign, CampaignVersion}, channels::{ChannelCurrent, InsertChannel, ChannelVersion}, users::{UserCurrent, PlayerCampaign, UserVersion, InsertUser}}};

/// The expected body contents of a POST request to create a new campaign.
#[derive(Debug, Deserialize)]
pub struct PostCampaignBody {
    /// The human-readable name of the campaign.
    pub name: String,
    /// The Id of the channel to which dice rolls are sent when invoked from
    /// the browser. (Slash commands will roll dice in the channel where they
    /// are invoked.)
    pub dice_channel: ChannelId,
    /// All channels that the campaign claims ownership of (including the dice
    /// channel)
    pub channels: HashSet<ChannelId>,
}

impl PostCampaignBody {
    /// Converts the body of a POST request into a ready-to-insert document.
    pub fn prepare_document(self, storyteller: UserId) -> InsertCampaign {
        // Make sure channels includes dice_channel
        let mut channels = self.channels;
        channels.insert(self.dice_channel);

        let players = [storyteller].into_iter().collect::<HashSet<UserId>>();
        
        
        InsertCampaign { 
            version: CampaignVersion::V0, 
            name: self.name, 
            storyteller, 
            players, 
            dice_channel: self.dice_channel,
            channels,
        }
    }
}

impl InsertCampaign {
    /// Inserts the campaign document into the database. Requires a session to 
    /// update campaigns, channels, and players atomically.
    pub async fn execute(&self, database: &mongodb::Database, session: &mut mongodb::ClientSession) -> Result<ObjectId, DatabaseError> {
                // Start a session to ensure all operations complete atomically
                session.start_transaction(None).await?;

                // Check if any of the channels here is already in use.
                let channel_ids: Vec<Bson> = self
                    .channels
                    .iter()
                    .map(|channel_id| bson::to_bson(channel_id).or_else(|_| Err(DatabaseError::SerializationError(format!{"{:?}", channel_id}))))
                    .fold(
                        Ok(Vec::new()),
                        |acc: Result<Vec<Bson>, DatabaseError>, res_bson| match (acc, res_bson) {
                            (Ok(mut v), Ok(b)) => {
                                v.push(b);
                                Ok(v)
                            }
                            (Err(e), _) => Err(e),
                            (_, Err(e)) => Err(e),
                        },
                    )?;
                let channels = database.collection::<ChannelCurrent>("channels");
                let filter = doc! {
                    "channelId": {
                        "$in": channel_ids
                    }
                };
                let maybe_channel = channels
                    .find_one_with_session(filter, None, session)
                    .await?;
                if let Some(channel) = maybe_channel {
                    return Err(DatabaseError::ConstraintError(ConstraintError::ChannelCampaignUnique(channel.channel_id)));
                }
        
                // Get a handle to the "campaigns" collection, but serializing document using InsertCampaign
                let campaigns = database.collection::<InsertCampaign>("campaigns");
        
                // Create a new document for this campaign
                let mongodb::results::InsertOneResult { inserted_id, .. } = campaigns
                    .insert_one_with_session(self, None, session)
                    .await?;
                let new_campaign_id = inserted_id
                    .as_object_id()
                    .ok_or_else(|| DatabaseError::DeserializationError("campaign_id".to_string()))?;
        
                // Get the storyteller's document, if it exists
                let users = database.collection::<UserCurrent>("users");
                let storyteller_bson = bson::to_bson(&self.storyteller).or_else(|_| Err(DatabaseError::SerializationError(format!{"{:?}", self.storyteller})))?;
                let filter = doc! {
                    "discordId": &storyteller_bson
                };
                let maybe_player = users.find_one_with_session(filter, None, session).await?;
        
                if let Some(mut player) = maybe_player {
                    // Update and replace the existing player document
                    if player.campaigns.iter().find(|player_campaign| player_campaign.campaign_id == new_campaign_id).is_none() {
                        player.campaigns.push(PlayerCampaign {
                            campaign_id: new_campaign_id,
                            name: self.name.clone(),
                            is_storyteller: true,
                            characters: Default::default(),
                            dice_channel: self.dice_channel,
                            channels: self.channels.clone(),
                        });
                    }
                    let query = doc! {
                        "discordId": &storyteller_bson
                    };
                    users.replace_one_with_session(query, player, None, session).await?;
                } else {
                    // Create a new user with this campaign.
                    let new_user = InsertUser {
                        version: UserVersion::V0,
                        discord_id: self.storyteller,
                        campaigns: vec![PlayerCampaign {
                            campaign_id: new_campaign_id,
                            name: self.name.clone(),
                            is_storyteller: true,
                            characters: Default::default(),
                            dice_channel: self.dice_channel,
                            channels: self.channels.clone(),
                        }]
                    };
                    let users = database.collection::<InsertUser>("users");
                    users.insert_one_with_session(new_user, None, session).await?;
                }
        
                // Create channel documents for all of the channels in this campaign
                let mut new_channels = Vec::new();
        
                for channel in self.channels.iter() {
                    let new_channel = InsertChannel {
                        version: ChannelVersion::V0,
                        channel_id: *channel,
                        campaign_id: new_campaign_id.clone(),
                    };
                    new_channels.push(new_channel);
                }
                let channels = database.collection::<InsertChannel>("channels");
                channels.insert_many_with_session(new_channels, None, session).await?;
        
                // Transaction complete, commit it
                session.commit_transaction().await?;
        
                Ok(new_campaign_id)
    }
}