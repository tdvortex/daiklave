use axum::{
    extract::{Path, State},
    Json,
};
use axum_extra::extract::SignedCookieJar;
use hyper::StatusCode;
use mongodb::bson::oid::ObjectId;
use serenity::all::UserId;

use crate::{
    api::{decode_user_id_cookie, WhyError},
    shared::{
        authorization::GetCampaignAuthorization,
        campaign::RemoveCampaignPlayer,
        error::{ConstraintError, DatabaseError},
    },
    AppState,
};

/// Handles a REST API request to remove a character from a campaign. The campaign ID
/// and user ID must be passed as querystring parameters
pub async fn delete_campaign_player(
    State(mut state): State<AppState>,
    jar: SignedCookieJar,
    Path((campaign_id, delete_id)): Path<(ObjectId, UserId)>,
) -> Result<StatusCode, (StatusCode, Json<WhyError>)> {
    // Get the user ID of the requester
    let requester_id = match decode_user_id_cookie(jar) {
        Ok(user_id) => user_id,
        Err(status_code) => {
            return Err((
                status_code,
                Json(WhyError {
                    why: "not logged in".to_owned(),
                }),
            ));
        }
    };

    // Get the requester's auth level
    let database = &state
        .mongodb_client
        .database(state.mongodb_database_name.as_str());
    let connection = &mut state.redis_connection_manager;

    let authorization_result = GetCampaignAuthorization {
        user_id: requester_id,
        campaign_id,
    }
    .execute(database, connection)
    .await;

    let authorization = match authorization_result {
        Ok(Some(authorization)) => authorization,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND, // Don't tell users about members of other campaigns
                Json(WhyError {
                    why: "not found or not authorized".to_owned(),
                }),
            ));
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(WhyError {
                    why: "internal server error".to_owned(),
                }),
            ));
        }
    };

    // Non-storyteller players can delete themselves from a campaign
    // Storytellers can delete other non-storyteller players from the campaign
    let is_authorized = authorization.user_id == requester_id
        && authorization.campaign_id == campaign_id
        && (delete_id == requester_id || authorization.is_storyteller);
    if !is_authorized {
        return Err((
            StatusCode::NOT_FOUND,
            Json(WhyError {
                why: "not found or not authorized".to_owned(),
            }),
        ));
    }

    // Storytellers cannot remove themselves
    if authorization.is_storyteller && delete_id == requester_id {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(WhyError {
                why: "cannot remove storyteller".to_owned(),
            }),
        ));
    }

    // Start a session for atomic updating
    let mut session = state.mongodb_client.start_session(None).await.map_err(|_| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(WhyError {
            why: "internal server error".to_owned(),
        }),
    ))?;

    // Try to remove the player from the camapign
    match (RemoveCampaignPlayer {
        campaign_id,
        user_id: delete_id,
    })
    .execute(database, &mut session, connection)
    .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(DatabaseError::ConstraintError(ConstraintError::RemoveStoryteller)) => Err((
            StatusCode::BAD_REQUEST,
            Json(WhyError {
                why: "cannot remove storyteller".to_owned(),
            }),
        )),
        _ => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(WhyError {
                why: "internal server error".to_owned(),
            }),
        )),
    }
}
