mod help;
mod set;
mod show;

use crate::{
    discord::interaction::{invalid_command_message, unknown_command_message},
    AppState,
};
use axum::response::Response;
use serenity::all::{CommandDataOptionValue, CommandInteraction};

use self::{
    help::campaign_channels_help, set::campaign_channels_set, show::campaign_channels_show,
};

pub async fn campaign_channels(interaction: &CommandInteraction, state: &mut AppState) -> Response {
    let channels_value = if let Some(option) = interaction.data.options.first() {
        match option.name.as_str() {
            "channels" => &option.value,
            other => {
                return unknown_command_message(&format!("campaign {}", other));
            }
        }
    } else {
        return invalid_command_message("/campaign requires a subcommand");
    };

    if let Some(channels_subcommand) = match channels_value {
        CommandDataOptionValue::SubCommandGroup(channels_group) => channels_group.first(),
        _ => {
            return invalid_command_message("/campaign channels should be a subcommand group");
        }
    } {
        match channels_subcommand.name.as_str() {
            "help" => campaign_channels_help(),
            "set" => campaign_channels_set(interaction, state).await,
            "show" => campaign_channels_show(interaction, state).await,
            other => unknown_command_message(&format!("campaign channels {}", other)),
        }
    } else {
        invalid_command_message("/campaign channels requires a subcommand")
    }
}
