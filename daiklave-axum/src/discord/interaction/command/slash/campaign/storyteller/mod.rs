mod help;
mod set;

use axum::response::Response;
use serenity::all::{CommandDataOptionValue, CommandInteraction};

use crate::{
    discord::interaction::{invalid_command_message, unknown_command_message},
    AppState,
};

use self::{help::campaign_storyteller_help, set::campaign_storyteller_set};

pub async fn campaign_storyteller(
    interaction: &CommandInteraction,
    state: &mut AppState,
) -> Response {
    let storyteller_value = if let Some(option) = interaction.data.options.first() {
        match option.name.as_str() {
            "storyteller" => &option.value,
            other => {
                return unknown_command_message(&format!("campaign {}", other));
            }
        }
    } else {
        return invalid_command_message("/campaign requires a subcommand");
    };

    if let Some(storyteller_subcommand) = match storyteller_value {
        CommandDataOptionValue::SubCommandGroup(channels_group) => channels_group.first(),
        _ => {
            return invalid_command_message("/campaign storyteller should be a subcommand group");
        }
    } {
        match storyteller_subcommand.name.as_str() {
            "help" => campaign_storyteller_help(),
            "set" => campaign_storyteller_set(interaction, state).await,
            other => unknown_command_message(&format!("campaign storyteller {}", other)),
        }
    } else {
        invalid_command_message("/campaign storyteller requires a subcommand")
    }
}
