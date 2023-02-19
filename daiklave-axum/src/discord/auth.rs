use mongodb::bson::oid::ObjectId;
use serenity::all::{ChannelId, UserId};

use crate::{
    shared::{authorization::GetChannelAuthorization, error::DatabaseError},
    AppState,
};

/// An enum representing the simplified state of a Discord user's auth for a
/// particular channel.
pub enum ChannelAuthResult {
    /// The user is not in this campaign at all.
    NotInCampaign,
    /// The user is a player who may or may not have an active character.
    Player {
        /// The id of the campaign.
        campaign_id: ObjectId,
        /// The player's active character, if any.
        active_character: Option<ObjectId>,
    },
    /// The user is the storyteller who may or may not have an active character.
    Storyteller {
        /// The id of the campaign.
        campaign_id: ObjectId,
        /// The storyteller's active character, if any.
        active_character: Option<ObjectId>,
    },
}

/// A helper function to quickly get the authorization for a user in this channel.
pub async fn get_channel_auth(
    state: &mut AppState,
    user_id: UserId,
    channel_id: ChannelId,
) -> Result<ChannelAuthResult, DatabaseError> {
    let database = state.mongodb_client.database(&state.mongodb_database_name);
    let connection = &mut state.redis_connection_manager;

    Ok(GetChannelAuthorization {
        user_id,
        channel_id,
    }
    .execute(&database, connection)
    .await?
    .map(|auth| {
        if auth.is_storyteller {
            ChannelAuthResult::Storyteller {
                campaign_id: auth.campaign_id,
                active_character: auth.active_character,
            }
        } else {
            ChannelAuthResult::Player {
                campaign_id: auth.campaign_id,
                active_character: auth.active_character,
            }
        }
    })
    .unwrap_or(ChannelAuthResult::NotInCampaign))
}
