use axum::response::Response;
use serenity::all::CommandInteraction;

use crate::AppState;

pub async fn campaign_leave(_interaction: &CommandInteraction, _state: &mut AppState) -> Response {
    todo!()
}
