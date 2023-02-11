/// Module for handling explicit slash commands.
pub mod slash;
use slash::post_slash;

use axum::response::Response;
use serenity::all::{CommandInteraction, CommandType};

use crate::AppState;

use super::unknown_command_message;

/// Handle a command interaction, which may be a slash command ("CHAT_INPUT"),
/// a user (right-click) interaction, or a message (right-click) interaction.
pub async fn post_command(interaction: &CommandInteraction, state: &mut AppState) -> Response {
    let data = &interaction.data;
    match data.kind {
        // aka slash command
        CommandType::ChatInput => post_slash(interaction, state).await,
        // No implementations; responding with a message but
        _ => unknown_command_message(&data.name),
    }
}
