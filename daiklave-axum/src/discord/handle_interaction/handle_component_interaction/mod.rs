use axum::{response::{IntoResponse, Response}, Json};
use serenity::{all::ComponentInteraction, builder::CreateInteractionResponse};

/// Handle an interaction on a message component. This may be a button click 
/// or an interaction with a select menu. It does NOT include text fields; text
/// fields only appear on modals (see [crate::discord::handle_interaction::handle_modal_interaction::handle_modal_interaction])
pub fn handle_component_interaction(_component_interaction: &ComponentInteraction) -> Response {
    // If we get an unexpected component interaction, can use 
    // DEFERRED_UPDATE_MESSAGE to tell Discord "acknowledged, do nothing"
    Json(CreateInteractionResponse::Acknowledge).into_response()
}