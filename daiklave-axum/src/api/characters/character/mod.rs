use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::SignedCookieJar;
use hyper::StatusCode;
use mongodb::bson::oid::ObjectId;

use crate::{
    api::{decode_user_id_cookie, internal_server_error, not_found, validate_player, WhyError},
    mongo::characters::CharacterCurrent,
    shared::{
        character::{DeleteCharacter, GetCharacter},
        error::DatabaseError,
    },
    AppState,
};

/// Handler for GET requests to retrieve a current character state.
pub async fn get_character(
    State(mut state): State<AppState>,
    jar: SignedCookieJar,
    Path((campaign_id, character_id)): Path<(ObjectId, ObjectId)>,
) -> Result<Json<CharacterCurrent>, (StatusCode, Json<WhyError>)> {
    let user_id = decode_user_id_cookie(jar)?;
    validate_player(&mut state, user_id, campaign_id).await?;

    let database = &state.mongodb_client.database(&state.mongodb_database_name);
    let connection = &mut state.redis_connection_manager;
    Ok(GetCharacter {
        player: user_id,
        campaign_id,
        character_id,
    }
    .execute(database, connection)
    .await
    .map_err(|_| internal_server_error())?
    .map(|character| Json(character))
    .ok_or_else(not_found)?)
}

/// Handler for DELETE requests to delete a current character.
pub async fn delete_character(
    State(mut state): State<AppState>,
    jar: SignedCookieJar,
    Path((campaign_id, character_id)): Path<(ObjectId, ObjectId)>,
) -> Result<StatusCode, (StatusCode, Json<WhyError>)> {
    let user_id = decode_user_id_cookie(jar)?;
    validate_player(&mut state, user_id, campaign_id).await?;
    let database = &state.mongodb_client.database(&state.mongodb_database_name);
    let session = &mut state
        .mongodb_client
        .start_session(None)
        .await
        .map_err(|_| internal_server_error())?;
    let connection = &mut state.redis_connection_manager;
    let delete_result = DeleteCharacter {
        player: user_id,
        campaign_id,
        character_id,
    }
    .execute(database, session, connection)
    .await;

    if let Err(e) = delete_result {
        match e {
            DatabaseError::NotFound(_) => Err(not_found()),
            _ => Err(internal_server_error()),
        }
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

/// TODO
pub async fn patch_character() -> impl IntoResponse {
    todo!()
}

/// TODO
pub async fn put_character() -> impl IntoResponse {
    todo!()
}
