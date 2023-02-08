use std::collections::HashSet;

use bson::{doc, oid::ObjectId, Bson};
use futures::TryStreamExt;
use mongodb::ClientSession;
use serenity::all::{ChannelId, UserId};

use crate::{
    campaign::{CampaignCurrent, CampaignDocument},
    character::CharacterCurrent,
    error::DocumentError,
    user::{CreateUser, UserCurrent, UserVersion},
    PlayerCampaign,
};

/// An instruction to change the storyteller and/or players for a campaign.
pub struct UpdateCampaignPlayers {
    /// The MongoDB database ID for the campaign.
    pub _id: ObjectId,
    /// The Discord Snowflake representing the storyteller for the campaign.
    pub storyteller: UserId,
    /// All the Discord snowflakes for the players in the campaign 
    /// (including the storyteller).
    pub players: HashSet<UserId>,
}

impl UpdateCampaignPlayers {
    /// Executes the command. Requires a transaction as it updates players, 
    /// campaigns, and characters atomically. Returns a vector of pairs of 
    /// (channel_id, player_id) permissions which must be invalidated
    /// in the cache.
    pub async fn execute(
        &self,
        database: &mongodb::Database,
        session: &mut ClientSession,
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
        let old_players = campaign.players.clone();
        let channels = campaign.channels.clone();

        // Get the players to add and drop from the campaign
        let drop_players = old_players
            .difference(&self.players)
            .copied()
            .collect::<Vec<UserId>>();
        let add_players = self
            .players
            .difference(&old_players)
            .copied()
            .collect::<Vec<UserId>>();

        let users = database.collection::<UserCurrent>("users");
        if !drop_players.is_empty() {
            let drop_players_bson = drop_players
                .iter()
                .map(|channel_id| {
                    bson::to_bson(channel_id).or(Err(DocumentError::SerializationError))
                })
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
            let drop_campaign_id =
                bson::to_bson(&self._id).or(Err(DocumentError::SerializationError))?;

            // Remove the campaign from the dropped players
            let query = doc! {
                "discordId": {
                    "$in": &drop_players_bson,
                },
            };
            let update = doc! {
                "$pull": {
                    "campaigns": {
                        "campaignId": &drop_campaign_id
                    }
                }
            };
            users
                .update_many_with_session(query, update, None, session)
                .await?;

            // Remove any characters tied to the dropped player-campaign pair
            let characters = database.collection::<CharacterCurrent>("characters");
            let query = doc! {
                "player": {
                    "$in": &drop_players_bson
                },
                "campaignId": &drop_campaign_id
            };
            characters
                .delete_many_with_session(query, None, session)
                .await?;
        }

        if !add_players.is_empty() {
            // Get all of the existing documents for these players
            let users = database.collection::<UserCurrent>("users");
            let user_ids = add_players
                .iter()
                .map(|discord_id| bson::to_bson(discord_id).unwrap_or(Bson::Int32(-1)))
                .collect::<Vec<Bson>>();
            let filter = doc! {
                "discordId" : {
                    "$in": user_ids
                }
            };
            let mut existing_users = users
                .find_with_session(filter, None, session)
                .await?
                .stream(session)
                .try_collect::<Vec<UserCurrent>>()
                .await?;
            let existing_user_ids = existing_users
                .iter()
                .map(|user| user.discord_id)
                .collect::<HashSet<UserId>>();

            // Create documents for any users that don't already have documents
            // Check to see if there are any
            let mut insert_ids = Vec::new();
            for snowflake in self.players.iter() {
                if !existing_user_ids.contains(snowflake) {
                    insert_ids.push(*snowflake);
                }
            }

            if !insert_ids.is_empty() {
                // Make a NewUser for each missing Id
                let insert_users = insert_ids.iter().map(|user_id| CreateUser {
                    version: UserVersion::V0,
                    discord_id: *user_id,
                });

                // Insert all of them at once
                let new_users = database.collection::<CreateUser>("users");
                new_users
                    .insert_many_with_session(insert_users, None, session)
                    .await?;

                // Retrieve just the newly inserted UserCurrent documents
                let new_ids_bson = insert_ids
                    .iter()
                    .map(|discord_id| bson::to_bson(discord_id).unwrap_or(Bson::Int32(-1)))
                    .collect::<Vec<Bson>>();
                let filter = doc! {
                    "discordId" : {
                        "$in": new_ids_bson
                    }
                };
                let created_users = users
                    .find(filter, None)
                    .await?
                    .try_collect::<Vec<UserCurrent>>()
                    .await?;

                // Add those to our existing users vec
                existing_users.extend(created_users.into_iter());
            }

            // Update all the players in the campaign to be a part of it
            // There is no "replace many" option in MongoDb, just chain replace_one
            for old_user in existing_users.into_iter() {
                let mut new_user = old_user.clone();
                let player_campaign = PlayerCampaign {
                    campaign_id: self._id.clone(),
                    name: campaign.name.clone(),
                    is_storyteller: self.storyteller == new_user.discord_id,
                    characters: Default::default(),
                    dice_channel: campaign.dice_channel,
                    channels: campaign.channels.clone(),
                };
                new_user.campaigns.push(player_campaign);

                users
                    .replace_one_with_session(
                        bson::to_document(&old_user).or(Err(DocumentError::SerializationError))?,
                        &new_user,
                        None,
                        session,
                    )
                    .await?;
            }
        }

        // Override the players in the campaign document
        let campaigns = database.collection::<CampaignDocument>("campaigns");
        let query = doc! {
            "_id": self._id
        };
        let update = doc! {
            "$set": {
                "storyteller": bson::to_bson(&self.storyteller).or(Err(DocumentError::SerializationError))?,
                "players": bson::to_bson(&self.players).or(Err(DocumentError::SerializationError))?,
            }
        };
        campaigns
            .update_one_with_session(query, update, None, session)
            .await?;

        // Done with the database
        session.commit_transaction().await?;

        // Any player who has been dropped from the campaign needs to have 
        // their cached permissions invalidated
        Ok(channels
            .into_iter()
            .flat_map(|channel_id| {
                drop_players
                    .iter()
                    .map(move |&user_id| (channel_id, user_id))
            })
            .collect::<Vec<(ChannelId, UserId)>>())
    }
}
