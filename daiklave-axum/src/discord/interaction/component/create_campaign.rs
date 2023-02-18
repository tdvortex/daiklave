use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serenity::{
    all::{ComponentInteraction, ComponentInteractionDataKind},
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
};

use crate::{
    discord::{components::create_campaign_message_components, partial::PartialCreateCampaign},
    shared::campaign::InsertCampaignRequest,
    AppState,
};

use super::acknowledge_component;

pub async fn create_campaign_components(
    component_interaction: &ComponentInteraction,
    state: &mut AppState,
) -> Response {
    // Load whatever we have saved in Redis for this interaction
    let old_partial: PartialCreateCampaign = match PartialCreateCampaign::load_partial(
        component_interaction.token.clone(),
        &mut state.redis_connection_manager,
    )
    .await
    {
        Ok(Some(partial)) => partial,
        _ => {
            return Json(CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("Internal server error"),
            ))
            .into_response();
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
                            return Json(CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new()
                                    .content("Submit failed: internal server error"),
                            ))
                            .into_response();
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
                        Json(CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content("Submit failed: internal server error"),
                        ))
                        .into_response()
                    }
                } else {
                    // This shouldn't happen, but if if does tell the user why we
                    // can't submit yet
                    Json(CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("Cannot submit yet: dice channel is required"),
                    ))
                    .into_response()
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
                .save_partial(
                    component_interaction.token.clone(),
                    &mut state.redis_connection_manager,
                )
                .await
                .is_ok()
            {
                create_campaign_message_components(enable_submit)
            } else {
                // Something went wrong in the database, tell the user
                Json(CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Submit failed: internal server error"),
                ))
                .into_response()
            }
        }
        _ => acknowledge_component(),
    }
}
