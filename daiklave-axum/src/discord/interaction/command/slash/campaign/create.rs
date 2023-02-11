use std::collections::HashSet;

use axum::{
    response::{IntoResponse, Response},
    Json,
};

use serenity::{
    all::{
        ButtonStyle, ChannelType, CommandDataOptionValue, CommandInteraction,
    },
    builder::{
        CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage,
        CreateSelectMenu, CreateSelectMenuKind,
    },
};

use crate::{AppState, discord::partial::PartialCreateCampaign};

pub async fn campaign_create(interaction: &CommandInteraction, state: &mut AppState) -> Response {
    let token = interaction.token.clone();
    let storyteller = interaction.user.id;
    let name = if let Some(name_option) = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "campaign_name")
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

    Json(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content("Choose the channels for this campaign:")
            .components(vec![
                CreateActionRow::SelectMenu(
                    CreateSelectMenu::new(
                        "create_dice_channel",
                        CreateSelectMenuKind::Channel {
                            channel_types: Some(vec![ChannelType::Text]),
                        },
                    )
                    .min_values(1)
                    .max_values(1)
                    .placeholder("Choose a dice channel"),
                ),
                CreateActionRow::SelectMenu(
                    CreateSelectMenu::new(
                        "create_all_channels",
                        CreateSelectMenuKind::Channel {
                            channel_types: Some(vec![ChannelType::Text]),
                        },
                    )
                    .min_values(1)
                    .max_values(25)
                    .placeholder("Choose campaign channels"),
                ),
                CreateActionRow::Buttons(vec![CreateButton::new("create_campaign_submit")
                    .label("Submit")
                    .disabled(true)
                    .style(ButtonStyle::Primary)]),
            ]),
    ))
    .into_response()
}


