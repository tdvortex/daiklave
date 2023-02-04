/// Module for handling explicit slash commands.
pub mod handle_slash_command;
use handle_slash_command::handle_slash_command;

use axum::response::Response;
use serenity::all::{CommandInteraction, CommandType};

use super::ok_empty_message;

/// Handle a command interaction, which may be a slash command ("CHAT_INPUT"),
/// a user (right-click) interaction, or a message (right-click) interaction.
pub fn handle_command_interaction(interaction: &CommandInteraction) -> Response {
    let data = &interaction.data;
    match data.kind {
        // aka slash command
        CommandType::ChatInput => handle_slash_command(interaction),
        // No implementations; responding with a message but
        _ => ok_empty_message(),
    }
}
