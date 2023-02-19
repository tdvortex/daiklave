use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serenity::{
    all::{ComponentInteraction, ComponentInteractionDataKind},
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
};

use crate::{
    discord::{
        components::set_channels_message,
        interaction::{internal_server_error, invalid_command_message},
        partial::PartialSetChannels,
    },
    shared::{campaign::SetCampaignChannels, error::DatabaseError},
    AppState,
};

use super::acknowledge_component;

pub async fn set_channels_components(
    component_interaction: &ComponentInteraction,
    state: &mut AppState,
) -> Response {
    // Load whatever we have saved in Redis for this interaction
    let old_partial: PartialSetChannels = match PartialSetChannels::load_partial(
        component_interaction.token.clone(),
        &mut state.redis_connection_manager,
    )
    .await
    {
        Ok(Some(partial)) => partial,
        _ => {
            return internal_server_error();
        }
    };

    match &component_interaction.data.kind {
        ComponentInteractionDataKind::Button => {
            if component_interaction.data.custom_id == "set_channels_submit" {
                // Make sure the partial is ready to submit
                if let Some(dice_channel) = old_partial.dice_channel {
                    // Submit approved
                    // Get a database and session handle for MongoDb
                    let database = state.mongodb_client.database(&state.mongodb_database_name);
                    let mut session =
                        if let Ok(session) = state.mongodb_client.start_session(None).await {
                            session
                        } else {
                            // Something went wrong in the database, tell the user
                            return internal_server_error();
                        };

                    let update_result = SetCampaignChannels {
                        campaign_id: old_partial.campaign_id,
                        dice_channel,
                        channels: old_partial.channels,
                    }
                    .execute(&database, &mut session, &mut state.redis_connection_manager)
                    .await;

                    match update_result {
                        Ok(_) => Json(CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::default()
                                .content("Channels updated successfully."),
                        ))
                        .into_response(),
                        Err(e) => match e {
                            DatabaseError::ConstraintError(c) => match c {
                                crate::shared::error::ConstraintError::ChannelCampaignUnique(
                                    channel_id,
                                ) => invalid_command_message(
                                    format!(
                                        "channel <#{}> is already in use by another campaign",
                                        channel_id.0
                                    )
                                    .as_str(),
                                ),
                                _ => internal_server_error(),
                            },
                            _ => internal_server_error(),
                        },
                    }
                } else {
                    // This shouldn't happen, but if if does tell the user why we
                    // can't submit yet
                    invalid_command_message("dice channel is required")
                }
            } else {
                // The only button on the message is "create_campaign_submit"
                acknowledge_component()
            }
        }
        ComponentInteractionDataKind::ChannelSelect { values } => {
            // Update the partial create value to reflect the new selections,
            // and check if it's now ready to submit
            let (new_partial, enable_submit) = match component_interaction.data.custom_id.as_str() {
                "set_dice_channel" => {
                    let mut new_partial = old_partial.clone();
                    if let Some(&channel_id) = values.first() {
                        new_partial.dice_channel = Some(channel_id);
                    }
                    (new_partial, true)
                }
                "set_all_channels" => {
                    let mut new_partial = old_partial.clone();
                    new_partial.channels = values.iter().copied().collect();
                    let enable_submit = new_partial.dice_channel.is_some();
                    (new_partial, enable_submit)
                }
                _ => {
                    return acknowledge_component();
                }
            };

            // Save the partial create value
            if new_partial
                .save_partial(
                    component_interaction.token.clone(),
                    &mut state.redis_connection_manager,
                )
                .await
                .is_ok()
            {
                set_channels_message(enable_submit)
            } else {
                // Something went wrong in the database, tell the user
                internal_server_error()
            }
        }
        _ => acknowledge_component(),
    }
}
