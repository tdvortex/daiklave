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
            .content(format!("Unknown command: /{}", command_name)),
    ))
    .into_response()
}

/// Response to Discord with a message to tell the user that the message is
/// recognized, but invalid somehow.
pub fn invalid_command_message(why: &str) -> Response {
    Json(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::default().content(format!("Invalid command: {}", why)),
    ))
    .into_response()
}

/// Response to Discord with a message just saying that something went wrong.
pub fn internal_server_error() -> Response {
    Json(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::default()
            .content("Internal server error: command could not be completed."),
    ))
    .into_response()
}

/// Response to Discord with a message saying that the user is not the
/// storyteller
pub fn forbidden() -> Response {
    Json(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::default()
            .content("Forbidden: only the storyteller is allowed to use this command."),
    ))
    .into_response()
}

/// Response to Discord with a message saying that the user is not in the
/// campaign
pub fn not_authorized() -> Response {
    Json(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::default()
            .content("Not authorized: you are not a player in this campaign."),
    ))
    .into_response()
}

/// Response to Discord with a message saying that the user has no active
/// character in this campaign.
pub fn no_active_character() -> Response {
    invalid_command_message("no active character for this campaign")
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
