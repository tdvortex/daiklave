use axum::{response::{IntoResponse, Response}, Json};
use serenity::{all::CommandInteraction, builder::{CreateInteractionResponse, CreateAutocompleteResponse}};

/// Handle an autocomplete interaction by querying the database to find 
/// appropriate fill-in-the-blank answers.
pub fn handle_autocomplete_interaction(_autocomplete_interaction: &CommandInteraction) -> Response {
    // If we get an unexpected autocomplete interaction, respond with an empty 
    // choices array
    Json(CreateInteractionResponse::Autocomplete(CreateAutocompleteResponse::new())).into_response()
}