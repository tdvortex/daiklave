use axum::Json;
use axum_extra::extract::SignedCookieJar;
use hyper::StatusCode;
use mongodb::bson::oid::ObjectId;
use serenity::all::UserId;

use crate::{
    shared::authorization::{Authorization, GetCampaignAuthorization},
    AppState,
};

use super::{
    internal_server_error, not_logged_in,
    not_found, not_storyteller,
    WhyError,
};

/// Utility function to try to get a UserId out of a SignedCookieJar. If
/// unsuccessful, returns 401 UNAUTHORIZED to force reauthentication.
pub fn decode_user_id_cookie(jar: SignedCookieJar) -> Result<UserId, (StatusCode, Json<WhyError>)> {
    // Try to get "daiklaveAuth" cookie from the jar
    let cookie = jar.get("daiklaveAuth").ok_or_else(not_logged_in)?;
    // Decode the hex string into byte vector
    let bytes_vec = hex::decode(cookie.value()).map_err(|_| not_logged_in())?;
    // Turn the byte vector into an 8-byte array

    let bytes_array = (bytes_vec.len() == 8)
        .then(|| {
            bytes_vec
                .into_iter()
                .enumerate()
                .fold([0; 8], |mut arr, (i, byte)| {
                    arr[i] = byte;
                    arr
                })
        })
        .ok_or_else(not_logged_in)?;

    // Turn the big-endian byte array into a u64
    // Turn the u64 into a UserId snowflake and return it
    Ok(UserId::from(u64::from_be_bytes(bytes_array)))
}

/// Get the full [Authorization] object for a user/campaign, or get a preformed
/// response of either 404 (if the campaign could not be found) or 500 (if
/// there was a connection error).
pub async fn get_auth(
    state: &mut AppState,
    user_id: UserId,
    campaign_id: ObjectId,
) -> Result<Authorization, (StatusCode, Json<WhyError>)> {
    let database = &state.mongodb_client.database(&state.mongodb_database_name);
    let connection = &mut state.redis_connection_manager;
    let authorization_request = GetCampaignAuthorization {
        user_id,
        campaign_id,
    }
    .execute(database, connection)
    .await;

    match authorization_request {
        Ok(Some(authorization)) => Ok(authorization),
        Ok(None) => Err(not_found()),
        Err(_) => Err(internal_server_error()),
    }
}

/// Checks that the user is a player in a campaign, returning Ok(()) if they
/// are.
pub async fn validate_player(
    state: &mut AppState,
    user_id: UserId,
    campaign_id: ObjectId,
) -> Result<(), (StatusCode, Json<WhyError>)> {
    get_auth(state, user_id, campaign_id).await?;
    Ok(())
}

/// Checks that the user is the storyteller in a campaign, returning Ok(())
/// if they are.
pub async fn validate_storyteller(
    state: &mut AppState,
    user_id: UserId,
    campaign_id: ObjectId,
) -> Result<(), (StatusCode, Json<WhyError>)> {
    if !get_auth(state, user_id, campaign_id).await?.is_storyteller {
        return Err(not_storyteller());
    }
    Ok(())
}
