use axum::{
    extract::{Path, State},
    Json,
};
use axum_extra::extract::SignedCookieJar;
use daiklave_core::{CharacterMemo, CharacterMutation};
use hyper::StatusCode;
use mongodb::bson::oid::ObjectId;

use crate::{
    api::{decode_user_id_cookie, internal_server_error, not_found, validate_player, WhyError},
    mongo::characters::CharacterCurrent,
    shared::{
        character::{DeleteCharacter, GetCharacter, PutCharacter, PatchCharacter},
        error::{DatabaseError, ConstraintError},
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

/// Handler for PATCH requests to apply a mutation to a character.
pub async fn patch_character(
    State(mut state): State<AppState>,
    jar: SignedCookieJar,
    Path((campaign_id, character_id)): Path<(ObjectId, ObjectId)>,
    Json(mutation): Json<CharacterMutation>
) -> Result<(), (StatusCode, Json<WhyError>)> {
    let user_id = decode_user_id_cookie(jar)?;
    validate_player(&mut state, user_id, campaign_id).await?;
    PatchCharacter {
        player: user_id,
        campaign_id,
        character_id,
        mutation,
    }.execute(&state.mongodb_client, &state.mongodb_database_name, &mut state.redis_connection_manager)
    .await
    .map_err(|e| match e {
        DatabaseError::ConstraintError(c) => if let ConstraintError::MutationError(m) = c {
            (StatusCode::BAD_REQUEST, Json(WhyError { why: format!("{:?}", m) }))
        } else {
            internal_server_error()
        }
        DatabaseError::NotFound(_) => not_found(),
        _ => internal_server_error()
    })
}

/// Handler for PUT requests to overwrite an existing character.
pub async fn put_character(
    State(mut state): State<AppState>,
    jar: SignedCookieJar,
    Path((campaign_id, character_id)): Path<(ObjectId, ObjectId)>,
    Json(character): Json<CharacterMemo>
) -> Result<(), (StatusCode, Json<WhyError>)> {
    let user_id = decode_user_id_cookie(jar)?;
    validate_player(&mut state, user_id, campaign_id).await?;
    let database = &state.mongodb_client.database(&state.mongodb_database_name);
    let session = &mut state
    .mongodb_client
    .start_session(None)
    .await
    .map_err(|_| internal_server_error())?;
    let connection = &mut state.redis_connection_manager;

    let put_result = PutCharacter {
        player: user_id,
        campaign_id,
        character_id,
        character,
    }.execute(database, session, connection)
    .await;

    if let Err(e) = put_result {
        match e {
            DatabaseError::NotFound(_) => Err(not_found()),
            _ => Err(internal_server_error()),
        }
    } else {
        Ok(())
    }
}
