/// The module for handling application command interactions. This includes
/// slash commands (aka "chat input"), user commands (from right clicking
/// users), and message commands (from right clicking messages).
pub mod handle_command_interaction;

/// The module for handling application autocomplete interactions.
pub mod handle_autocomplete_interaction;

/// The module for handling interactions on message components 
/// (buttons and select dropdowns).
pub mod handle_component_interaction;

/// The module for handling application submission of text input via modal 
/// popup.
pub mod handle_modal_interaction;

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use handle_autocomplete_interaction::handle_autocomplete_interaction;
use handle_component_interaction::handle_component_interaction;
use handle_command_interaction::handle_command_interaction;
use handle_modal_interaction::handle_modal_interaction;
use serenity::{
    all::Interaction,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
};

/// Response to Discord with a message to tell the user that the message is 
/// unrecognized, just to prevent an empty screen or a vague "message failed".
pub fn unknown_command_message(command_name: &str) -> Response {
    Json(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::default()
            .content(format!("Unknown command: {}", command_name)),
    ))
    .into_response()
}

/// Handles an interaction base on its type.
pub fn handle_interaction(interaction: &Interaction) -> Response {
    match &interaction {
        Interaction::Ping(_) => Json(CreateInteractionResponse::Pong).into_response(),
        Interaction::Command(command_interaction) => {
            handle_command_interaction(command_interaction)
        }
        Interaction::Autocomplete(autocomplete_interaction) => handle_autocomplete_interaction(autocomplete_interaction),
        Interaction::Component(component_interaction) => handle_component_interaction(component_interaction),
        Interaction::Modal(modal_submit) => handle_modal_interaction(modal_submit),
    }
}
