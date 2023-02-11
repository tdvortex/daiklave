mod create;
use axum::response::Response;
pub use create::campaign_create;
use serenity::all::CommandInteraction;

use crate::{AppState, discord::interaction::unknown_command_message};

pub async fn campaign(interaction: &CommandInteraction, state: &mut AppState) -> Response {
    if let Some(option) = interaction.data.options.first() {
        match option.name.as_str() {
            "campaign_create" => campaign_create(interaction, state).await,
            other => unknown_command_message(other)
        }
    } else {
        unknown_command_message(interaction.data.name.as_str())
    }
}