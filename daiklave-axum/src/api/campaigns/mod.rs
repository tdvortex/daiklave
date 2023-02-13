/// Routes related to a specific campaign.
pub mod campaign;

use axum::http::StatusCode;
use axum::{extract::State, Json};
use axum_extra::extract::SignedCookieJar;

use crate::{mongo::users::PlayerCampaign, shared::campaign::ListCampaigns, AppState};

use super::{decode_user_id_cookie, internal_server_error, WhyError};

/// Returns a list of campaigns for an authenticated player as a JSON payload.
/// Returns 401 UNAUTHORIZED if the user is not logged in, or 500 INTERNAL
/// SERVER ERROR if the database lookup fails for any reason.
pub async fn get_campaigns(
    State(state): State<AppState>,
    jar: SignedCookieJar,
) -> Result<Json<Vec<PlayerCampaign>>, (StatusCode, Json<WhyError>)> {
    let user_id = decode_user_id_cookie(jar)?;
    let database = state.mongodb_client.database(&state.mongodb_database_name);

    match (ListCampaigns { user_id }).execute(&database).await {
        Ok(player_campaigns) => Ok(Json(player_campaigns)),
        Err(_) => Err(internal_server_error()),
    }
}

// #[derive(Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PostCampaignSuccess {
//     campaign_id: ObjectId,
// }

// #[derive(Serialize)]
// pub struct PostCampaignError {
//     why: String,
// }

// /// Creates a campaign based on the provided message body, with the user ID
// /// of the creator as the default storyteller. If successful, returns JSON with
// /// the created campaign Id. If unsuccessful, may return a 401 UNAUTHORIZED,
// /// 400 BAD REQUEST, or 500 INTERNAL SERVER ERROR with a "why" JSON response.
// pub async fn create_campaign(
//     State(state): State<AppState>,
//     jar: SignedCookieJar,
//     Json(post_campaign_body): Json<PostCampaignBody>,
// ) -> Result<Json<PostCampaignSuccess>, (StatusCode, Json<PostCampaignError>)> {
//     let user_id = match decode_user_id_cookie(jar) {
//         Ok(user_id) => user_id,
//         Err(status_code) => {
//             return Err((
//                 status_code,
//                 Json(PostCampaignError {
//                     why: "not logged in".to_owned(),
//                 }),
//             ));
//         }
//     };

//     let database = state.mongodb_client.database(&state.mongodb_database_name);
//     let mut session = state
//         .mongodb_client
//         .start_session(None)
//         .await
//         .or_else(|_| {
//             Err((
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(PostCampaignError {
//                     why: "internal error".to_owned(),
//                 }),
//             ))
//         })?;

//     match post_campaign_body
//         .prepare_document(user_id)
//         .execute(&database, &mut session)
//         .await
//     {
//         Ok(campaign_id) => Ok(Json(PostCampaignSuccess {
//             campaign_id,
//         })),
//         Err(e) => match e {
//             DatabaseError::ConstraintError(constraint) => Err((
//                 StatusCode::BAD_REQUEST,
//                 Json(PostCampaignError {
//                     why: format!("{:?}", constraint),
//                 }),
//             )),
//             _ => Err((
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(PostCampaignError {
//                     why: "internal error".to_owned(),
//                 }),
//             )),
//         },
//     }
// }
