mod channels;
mod create;
mod help;
mod join;
mod kick;
mod leave;
mod rename;
mod storyteller;
use axum::response::Response;
pub use create::campaign_create;
use serenity::all::CommandInteraction;

use crate::{AppState, discord::interaction::unknown_command_message};

use self::{channels::campaign_channels, help::campaign_help, join::campaign_join, kick::campaign_kick, leave::campaign_leave, rename::campaign_rename, storyteller::campaign_storyteller};

pub async fn campaign(interaction: &CommandInteraction, state: &mut AppState) -> Response {
    if let Some(option) = interaction.data.options.first() {
        match option.name.as_str() {
            "channels" => campaign_channels(interaction, state).await,
            "create" => campaign_create(interaction, state).await,
            "help" => campaign_help(),
            "join" => campaign_join(interaction, state).await,
            "kick" => campaign_kick(interaction, state).await,
            "leave" => campaign_leave(interaction, state).await,
            "rename" => campaign_rename(interaction, state).await,
            "storyteller" => campaign_storyteller(interaction, state).await,
            other => unknown_command_message(&format!("campaign {}", other))
        }
    } else {
        unknown_command_message(interaction.data.name.as_str())
    }
}