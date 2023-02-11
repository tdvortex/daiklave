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
    discord::{components::create_campaign_message_components, partial::PartialCreateCampaign},
    AppState,
};

pub async fn campaign_create(interaction: &CommandInteraction, state: &mut AppState) -> Response {
    let token = interaction.token.clone();
    let storyteller = interaction.user.id;

    let subcommand = if let Some(subcommand) = interaction.data.options.first() {
        subcommand
    } else {
        return Json(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content("Campaign not created: malformed request"),
        ))
        .into_response(); 
    };

    let params = match &subcommand.value {
        CommandDataOptionValue::SubCommand(params) => params,
        _ => {return Json(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content("Campaign not created: malformed request"),
        ))
        .into_response(); 
        },
    };

    let name = if let Some(name_option) = params
        .iter()
        .find(|option| option.name == "name")
    {
        match &name_option.value {
            CommandDataOptionValue::String(name) => name.to_owned(),
            _ => {
                return Json(CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Campaign not created: campaign name must be a string"),
                ))
                .into_response();
            }
        }
    } else {
        return Json(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content("Campaign not created: campaign name is required"),
        ))
        .into_response();
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
