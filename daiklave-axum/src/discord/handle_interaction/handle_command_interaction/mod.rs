/// Module for handling explicit slash commands.
pub mod handle_slash_command;
use handle_slash_command::handle_slash_command;

use axum::response::Response;
use serenity::all::{CommandInteraction, CommandType};

use crate::AppState;

use super::unknown_command_message;

/// Handle a command interaction, which may be a slash command ("CHAT_INPUT"),
/// a user (right-click) interaction, or a message (right-click) interaction.
pub fn handle_command_interaction(interaction: &CommandInteraction, state: &AppState) -> Response {
    let data = &interaction.data;
    match data.kind {
        // aka slash command
        CommandType::ChatInput => handle_slash_command(interaction, state),
        // No implementations; responding with a message but
        _ => unknown_command_message(&data.name),
    }
}
