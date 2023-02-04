/// The module for handling application command interactions. This includes
/// slash commands (aka "chat input"), user commands (from right clicking
/// users), and message commands (from right clicking messages).
pub mod handle_command_interaction;

use axum::{response::{Response, IntoResponse}, Json};
use handle_command_interaction::handle_command_interaction;
use serenity::{all::Interaction, builder::{CreateInteractionResponse, CreateInteractionResponseMessage}};

/// Creates a response to Discord with a status code of 200 OK, a type of
/// ChannelMessageWithSource, but an empty body so no message is actually 
/// added.
pub fn ok_empty_message() -> Response {
    Json(CreateInteractionResponse::Message(CreateInteractionResponseMessage::default())).into_response()
}

/// Handles an interaction base on its type.
pub fn handle_interaction(interaction: &Interaction) -> Response {
    match &interaction {
        Interaction::Ping(_) => Json(CreateInteractionResponse::Pong).into_response(),
        Interaction::Command(command_interaction) => handle_command_interaction(&command_interaction),
        Interaction::Autocomplete(_) => ().into_response(),
        Interaction::Component(_) => ().into_response(),
        Interaction::Modal(_) => ().into_response(),
    }
}