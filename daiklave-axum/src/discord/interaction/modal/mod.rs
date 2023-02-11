use axum::{Json, response::{Response, IntoResponse}};
use serenity::{all::ModalInteraction, builder::CreateInteractionResponse};

use crate::AppState;

/// Handle the closing of a modal interaction. This occurs when a user has 
/// filled out all text fields and has submitted it.
pub fn post_modal(_modal_submit: &ModalInteraction, _state: &AppState) -> Response {
    // If we get an unexpected modal submit interaction, can use 
    // DEFERRED_UPDATE_MESSAGE to tell Discord "acknowledged, do nothing"
    Json(CreateInteractionResponse::Acknowledge).into_response()
}