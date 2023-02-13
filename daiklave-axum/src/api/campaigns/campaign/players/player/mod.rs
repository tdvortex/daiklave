use axum::{
    extract::{Path, State},
    Json, 
};
use axum_extra::extract::SignedCookieJar;
use hyper::StatusCode;
use mongodb::bson::oid::ObjectId;
use serenity::all::UserId;

use crate::{
    api::{decode_user_id_cookie, WhyError, not_found, internal_server_error, get_auth},
    shared::{
        campaign::RemoveCampaignPlayer,
        error::{ConstraintError, DatabaseError},
    },
    AppState,
};

fn cannot_remove_storyteller() -> (StatusCode, Json<WhyError>) {
    (
        StatusCode::BAD_REQUEST,
        Json(WhyError {
            why: "cannot remove storyteller".to_owned(),
        }),
    )
}


/// Handles a REST API request to remove a character from a campaign. The campaign ID
/// and user ID must be passed as querystring parameters
pub async fn delete_campaign_player(
    State(mut state): State<AppState>,
    jar: SignedCookieJar,
    Path((campaign_id, delete_id)): Path<(ObjectId, UserId)>,
) -> Result<StatusCode, (StatusCode, Json<WhyError>)> {
    // Get the user ID of the requester
    let requester_id = decode_user_id_cookie(jar)?;

    // Get the requester's auth level
    let authorization = get_auth(&mut state, requester_id, campaign_id).await?;

    // Non-storyteller players can delete themselves from a campaign
    // Storytellers can delete other non-storyteller players from the campaign
    let is_authorized = authorization.user_id == requester_id
        && authorization.campaign_id == campaign_id
        && (delete_id == requester_id || authorization.is_storyteller);
    if !is_authorized {
        return Err(not_found());
    }

    // Storytellers cannot remove themselves
    if authorization.is_storyteller && delete_id == requester_id {
        return Err(cannot_remove_storyteller());
    }

    // Try to remove the player from the campaign
    let database = &state.mongodb_client.database(&state.mongodb_database_name);
    let mut session = state.mongodb_client.start_session(None).await.map_err(|_| internal_server_error())?;
    let connection = &mut state.redis_connection_manager;
    match (RemoveCampaignPlayer {
        campaign_id,
        user_id: delete_id,
    })
    .execute(database, &mut session, connection)
    .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(DatabaseError::ConstraintError(ConstraintError::RemoveStoryteller)) => Err(cannot_remove_storyteller()),
        _ => Err(internal_server_error()),
    }
}
