mod set_concept;
pub use set_concept::set_concept_modal;

use axum::response::Response;
use serenity::all::ModalInteraction;

use crate::AppState;

use super::acknowledge_component;

/// Handle the closing of a modal interaction. This occurs when a user has
/// filled out all text fields and has submitted it.
pub async fn post_modal(modal_submit: &ModalInteraction, state: &mut AppState) -> Response {
    match modal_submit.data.custom_id.as_str() {
        "set_concept" => set_concept_modal(modal_submit, state).await,
        // If we get an unexpected modal submit interaction, can use
        // DEFERRED_UPDATE_MESSAGE to tell Discord "acknowledged, do nothing"
        _ => acknowledge_component(),
    }
}
