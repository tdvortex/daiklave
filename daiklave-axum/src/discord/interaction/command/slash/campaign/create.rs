use std::collections::HashSet;

use axum::{
    response::{IntoResponse, Response},
    Json,
};

use serenity::{
    all::{CommandDataOptionValue, CommandInteraction},
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
};

use crate::{
    discord::{components::create_campaign_message_components, partial::PartialCreateCampaign, interaction::invalid_command_message},
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
    if let Err(_) = partial_create_campaign
        .save_partial(token, connection)
        .await
    {
        return Json(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content("Campaign not created: internal server error"),
        ))
        .into_response();
    }

    create_campaign_message_components(false)
}
