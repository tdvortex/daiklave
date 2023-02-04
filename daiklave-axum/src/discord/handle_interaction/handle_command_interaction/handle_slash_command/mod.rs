use axum::response::{Response, IntoResponse};
use serenity::all::CommandInteraction;

use crate::discord::handle_interaction::ok_empty_message;

/// Handle a slash command (of type CHAT INPUT).
pub fn handle_slash_command(interaction: &CommandInteraction) -> Response {
    // Exact data needs may vary by command, but we need the name to route it
    let command_name = interaction.data.name.as_str();

    match command_name {
        "version" => "Daiklave version 0.1.0".into_response(),
        _ => ok_empty_message(),
    }
}