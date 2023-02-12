mod help;
mod set;

use axum::response::Response;
use serenity::all::{CommandInteraction, CommandDataOptionValue};

use crate::{AppState, discord::interaction::unknown_command_message};

use self::{help::campaign_storyteller_help, set::campaign_storyteller_set};

pub async fn campaign_storyteller(interaction: &CommandInteraction, state: &mut AppState) -> Response {
    let storyteller_value = if let Some(option) = interaction.data.options.first() {
        match option.name.as_str() {
            "storyteller" => &option.value,
            other => {return unknown_command_message(&format!("campaign {}", other));}
        }
    } else {
        return unknown_command_message(interaction.data.name.as_str());
    };

    if let Some(storyteller_subcommand) = match storyteller_value {
        CommandDataOptionValue::SubCommandGroup(channels_group) => channels_group.first(),
        _ => {
            return unknown_command_message(interaction.data.name.as_str());
        }
    } {
        match storyteller_subcommand.name.as_str() {
            "help" => campaign_storyteller_help(),
            "set" => campaign_storyteller_set(interaction, state).await,
            other => unknown_command_message(&format!("campaign storyteller {}", other)),
        }
    } else {
        unknown_command_message("campaign channels")
    }
}