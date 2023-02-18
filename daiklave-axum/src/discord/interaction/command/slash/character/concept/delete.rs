use axum::response::Response;
use serenity::all::CommandInteraction;

use crate::AppState;

pub async fn character_concept_delete(
    _interaction: &CommandInteraction,
    _state: &mut AppState,
) -> Response {
    todo!()
}
