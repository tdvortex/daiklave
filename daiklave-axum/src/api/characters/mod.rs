use axum::{extract::{State, Path}, Json};
use axum_extra::extract::SignedCookieJar;
use daiklave_core::CharacterMemo;
use hyper::StatusCode;
use mongodb::bson::oid::ObjectId;

use crate::{AppState, shared::{character::CreateCharacter}};

use super::{decode_user_id_cookie, validate_player, WhyError, internal_server_error};

/// Routes related to a specific character. 
pub mod character;

/// Handler for POST requests to create a new character.
pub async fn post_character(
    State(mut state): State<AppState>,
    jar: SignedCookieJar,
    Path(campaign_id): Path<ObjectId>,
    Json(character): Json<CharacterMemo>
) -> Result<Json<ObjectId>, (StatusCode, Json<WhyError>)> {
    let user_id = decode_user_id_cookie(jar)?;
    validate_player(&mut state, user_id, campaign_id).await?;
    let database = &state.mongodb_client.database(&state.mongodb_database_name);
    let session = &mut state
    .mongodb_client
    .start_session(None)
    .await
    .map_err(|_| internal_server_error())?;

    let post_result = CreateCharacter {
        player: user_id,
        campaign_id,
        character,
    }.execute(database, session)
    .await;

    if let Ok(id) = post_result {
        Ok(Json(id))
    } else {
        Err(internal_server_error())
    }    
}