/// The module for handling application command interactions. This includes
/// slash commands (aka "chat input"), user commands (from right clicking
/// users), and message commands (from right clicking messages).
pub mod command;

/// The module for handling application autocomplete interactions.
pub mod autocomplete;

/// The module for handling interactions on message components
/// (buttons and select dropdowns).
pub mod component;

/// The module for handling application submission of text input via modal
/// popup.
pub mod modal;

use autocomplete::post_autocomplete;
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use command::post_command;
use component::post_component;
use modal::post_modal;
use serenity::{
    all::Interaction,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
};

use crate::AppState;

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
pub async fn post_interaction(interaction: &Interaction, state: &mut AppState) -> Response {
    match &interaction {
        Interaction::Ping(_) => Json(CreateInteractionResponse::Pong).into_response(),
        Interaction::Command(command_interaction) => post_command(command_interaction, state).await,
        Interaction::Autocomplete(autocomplete_interaction) => {
            post_autocomplete(autocomplete_interaction, state)
        }
        Interaction::Component(component_interaction) => {
            post_component(component_interaction, state).await
        }
        Interaction::Modal(modal_submit) => post_modal(modal_submit, state),
    }
}
