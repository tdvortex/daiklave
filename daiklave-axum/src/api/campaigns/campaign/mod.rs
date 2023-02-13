use axum::{ extract::{State, Path}, Json};
use axum_extra::extract::SignedCookieJar;
use hyper::StatusCode;
use mongodb::bson::oid::ObjectId;

use crate::{AppState, mongo::users::PlayerCampaign, api::{WhyError, decode_user_id_cookie, get_auth, internal_server_error, not_found}, shared::{campaign::GetCampaign}};
pub use crate::api::characters;

/// Routes related to managing the players of a campaign.
pub mod players;

/// Gets the full details of a campaign from the perspective of a player. 
pub async fn get_campaign(State(mut state): State<AppState>, jar: SignedCookieJar, Path(campaign_id): Path<ObjectId>) -> Result<Json<PlayerCampaign>, (StatusCode, Json<WhyError>)> {
    // Get the user ID of the requester
    let user_id = decode_user_id_cookie(jar)?;

    // Make sure they have player auth for this 
    get_auth(&mut state, user_id, campaign_id).await?;

    let get_result = GetCampaign {
        user_id,
        campaign_id,
    }.execute(&state.mongodb_client.database(&state.mongodb_database_name)).await;

    match get_result {
        Ok(Some(player_campaign)) => Ok(Json(player_campaign)),
        Ok(None) => Err(not_found()),
        Err(_) => Err(internal_server_error()),
    }
}