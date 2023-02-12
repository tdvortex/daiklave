use axum::response::IntoResponse;

pub use crate::api::characters;

/// Routes related to managing the players of a campaign.
pub mod players;

/// TODO
pub async fn get_campaign() -> impl IntoResponse {
    todo!()
}