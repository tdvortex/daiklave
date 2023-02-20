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
        components::create_campaign_message,
        interaction::{acknowledge_component, internal_server_error, invalid_command_message},
        partial::PartialCreateCampaign,
    },
    shared::campaign::InsertCampaignRequest,
    AppState,
};

pub async fn create_campaign_components(
    component_interaction: &ComponentInteraction,
    state: &mut AppState,
) -> Response {
    // Load whatever we have saved in Redis for this interaction
    let old_partial: PartialCreateCampaign = match PartialCreateCampaign::load(
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
            if component_interaction.data.custom_id == "create_campaign_submit" {
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

                    if (InsertCampaignRequest {
                        name: old_partial.name.clone(),
                        storyteller: old_partial.storyteller,
                        dice_channel,
                        channels: old_partial.channels,
                    })
                    .into_document()
                    .execute(&database, &mut session)
                    .await
                    .is_ok()
                    {
                        // Replace the completed message to prevent extra button presses
                        Json(CreateInteractionResponse::UpdateMessage(
                            CreateInteractionResponseMessage::new().content(format!(
                                "Campaign \"{}\" created successfully",
                                old_partial.name
                            )),
                        ))
                        .into_response()
                    } else {
                        // Something went wrong in the database, tell the user
                        internal_server_error()
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
                "create_dice_channel" => {
                    let mut new_partial = old_partial.clone();
                    if let Some(&channel_id) = values.first() {
                        new_partial.dice_channel = Some(channel_id);
                    }
                    (new_partial, true)
                }
                "create_all_channels" => {
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
                .save(
                    component_interaction.token.clone(),
                    &mut state.redis_connection_manager,
                )
                .await
                .is_ok()
            {
                create_campaign_message(enable_submit)
            } else {
                // Something went wrong in the database, tell the user
                internal_server_error()
            }
        }
        _ => acknowledge_component(),
    }
}
