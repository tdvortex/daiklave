use axum::{Json, extract::State};
use axum_extra::extract::SignedCookieJar;
use axum::http::StatusCode;

use crate::{mongo::users::PlayerCampaign, shared::campaign::ListCampaigns, AppState};

use super::decode_user_id_cookie;

/// Returns a list of campaigns for an authenticated player as a JSON payload.
/// Returns 401 UNAUTHORIZED if the user is not logged in, or 500 INTERNAL 
/// SERVER ERROR if the database lookup fails for any reason.
pub async fn list_campaigns(State(state): State<AppState>, jar: SignedCookieJar) -> Result<Json<Vec<PlayerCampaign>>, StatusCode> {
    let user_id = decode_user_id_cookie(jar)?;
    let database = state.mongodb_client.database(&state.mongodb_database_name);

    match (ListCampaigns {
        user_id,
    }).execute(&database).await {
        Ok(player_campaigns) => Ok(Json(player_campaigns)),
        Err(e) => {
            tracing::error!("Data error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}