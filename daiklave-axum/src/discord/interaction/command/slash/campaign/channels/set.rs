use axum::response::Response;
use serenity::all::CommandInteraction;

use crate::AppState;

pub async fn campaign_channels_set(
    _interaction: &CommandInteraction,
    _state: &mut AppState,
) -> Response {
    todo!()
}
