mod campaign;
mod character;
mod help;
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use campaign::{campaign};
use character::character;
use help::help;
use serenity::{
    all::CommandInteraction,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
};

use crate::{discord::interaction::unknown_command_message, AppState};

/// Handle a slash command (of type CHAT INPUT).
pub async fn post_slash(interaction: &CommandInteraction, state: &mut AppState) -> Response {
    // Exact data needs may vary by command, but we need the name to route it
    let command_name = interaction.data.name.as_str();

    match command_name {
        "campaign" => campaign(interaction, state).await,
        "character" => character(interaction, state).await,
        "help" => help(),
        "version" => Json(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content("Daiklave version 0.1.0"),
        ))
        .into_response(),
        // We don't have support for this command yet
        other_name => unknown_command_message(other_name),
    }
}