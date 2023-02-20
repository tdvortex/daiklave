use std::collections::HashSet;

use axum::response::Response;

use serenity::all::{CommandDataOptionValue, CommandInteraction};

use crate::{
    discord::{
        components::create_campaign_message,
        interaction::{internal_server_error, invalid_command_message},
        partial::PartialCreateCampaign,
    },
    AppState,
};

pub async fn campaign_create(interaction: &CommandInteraction, state: &mut AppState) -> Response {
    let token = interaction.token.clone();
    let storyteller = interaction.user.id;

    let subcommand = if let Some(subcommand) = interaction.data.options.first() {
        subcommand
    } else {
        return invalid_command_message("/campaign requires a subcommand");
    };

    let params = match &subcommand.value {
        CommandDataOptionValue::SubCommand(params) => params,
        _ => {
            return invalid_command_message("/campaign create should be a subcommand");
        }
    };

    let name = if let Some(name_option) = params.iter().find(|option| option.name == "name") {
        match &name_option.value {
            CommandDataOptionValue::String(name) => name.to_owned(),
            _ => {
                return invalid_command_message("Campaign name must be a string");
            }
        }
    } else {
        return invalid_command_message("Campaign name is required");
    };

    let partial_create_campaign = PartialCreateCampaign {
        name,
        storyteller,
        dice_channel: None,
        channels: HashSet::new(),
    };
    let connection = &mut state.redis_connection_manager;
    if let Err(_) = partial_create_campaign.save(token, connection).await {
        return internal_server_error();
    }

    create_campaign_message(false)
}
