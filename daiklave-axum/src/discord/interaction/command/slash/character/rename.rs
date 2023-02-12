use axum::response::Response;
use serenity::all::CommandInteraction;

use crate::AppState;

pub async fn character_rename(_interaction: &CommandInteraction, _state: &mut AppState) -> Response {
    todo!()
}