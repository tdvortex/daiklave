use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serenity::{
    all::CommandInteraction,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
};

use crate::{discord::handle_interaction::unknown_command_message, AppState};

/// Handle a slash command (of type CHAT INPUT).
pub fn handle_slash_command(interaction: &CommandInteraction, _state: &AppState) -> Response {
    // Exact data needs may vary by command, but we need the name to route it
    let command_name = interaction.data.name.as_str();

    match command_name {
        "version" => Json(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content("Daiklave version 0.1.0"),
        ))
        .into_response(),
        // We don't have support for this command yet
        other_name => unknown_command_message(other_name),
    }
}