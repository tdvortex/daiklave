use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serenity::{
    all::{ButtonStyle, ChannelType},
    builder::{
        CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage,
        CreateSelectMenu, CreateSelectMenuKind,
    },
};

/// Creates a message component interaction for setting the channels for a
/// campaign.
/// Action row 1: a text channel select (min 1, max 1) for the dice channel
/// Action row 2: a text channel select (min 0, max 25) for other channels
/// Action row 3: a Submit button
pub fn set_channels_message(enable_submit: bool) -> Response {
    Json(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content("Choose the channels for this campaign:")
            .components(vec![
                CreateActionRow::SelectMenu(
                    CreateSelectMenu::new(
                        "set_dice_channel",
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
                        "set_all_channels",
                        CreateSelectMenuKind::Channel {
                            channel_types: Some(vec![ChannelType::Text]),
                        },
                    )
                    .min_values(0)
                    .max_values(25)
                    .placeholder("Choose campaign channels"),
                ),
                CreateActionRow::Buttons(vec![CreateButton::new("set_channels_submit")
                    .label("Submit")
                    .disabled(!enable_submit)
                    .style(ButtonStyle::Primary)]),
            ]),
    ))
    .into_response()
}
