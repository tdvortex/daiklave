use mongodb::bson::oid::ObjectId;
use serenity::all::{UserId, ChannelId};

use crate::{shared::{error::DatabaseError, authorization::GetChannelAuthorization}, AppState};

/// An enum representing the simplified state of a Discord user's auth for a
/// particular channel.
pub enum ChannelAuthResult {
    /// The user is not in this campaign at all.
    NotInCampaign,
    /// The user is a player who may or may not have an active character.
    Player(Option<ObjectId>),
    /// The user is the storyteller who may or may not have an active character.
    Storyteller(Option<ObjectId>),
}

/// A helper function to quickly get the authorization for a user in this channel.
pub async fn get_channel_auth(state: &mut AppState, user_id: UserId, channel_id: ChannelId) -> Result<ChannelAuthResult, DatabaseError> {
    let database = state.mongodb_client.database(&state.mongodb_database_name);
    let connection = &mut state.redis_connection_manager;
    
    
    Ok(GetChannelAuthorization {
            user_id,
            channel_id,
        }.execute(&database, connection).await?.map(|auth| if auth.is_storyteller {
            ChannelAuthResult::Storyteller(auth.active_character)
        } else {
            ChannelAuthResult::Player(auth.active_character)
        }).unwrap_or(ChannelAuthResult::NotInCampaign))
}