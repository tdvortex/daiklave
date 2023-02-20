use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serenity::{
    all::ComponentInteraction,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
};

use crate::{
    discord::{
        interaction::{acknowledge_component, internal_server_error, invalid_command_message},
        partial::PartialLeaveCampaign,
    },
    shared::campaign::RemoveCampaignPlayer,
    AppState,
};

pub async fn leave_campaign_components(
    component_interaction: &ComponentInteraction,
    state: &mut AppState,
) -> Response {
    // Load whatever we have saved in Redis for this interaction
    let old_partial: PartialLeaveCampaign = match PartialLeaveCampaign::load(
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
        serenity::all::ComponentInteractionDataKind::Button => {
            match component_interaction.data.custom_id.as_str() {
                "leave_campaign_confirm" => {
                    let database = state.mongodb_client.database(&state.mongodb_database_name);
                    let mut session =
                        if let Ok(session) = state.mongodb_client.start_session(None).await {
                            session
                        } else {
                            return internal_server_error();
                        };

                    let remove_result = (RemoveCampaignPlayer {
                        campaign_id: old_partial.campaign_id,
                        user_id: old_partial.user_id,
                    })
                    .execute(&database, &mut session, &mut state.redis_connection_manager)
                    .await;

                    match remove_result {
                        Ok(_) => Json(CreateInteractionResponse::UpdateMessage(
                            CreateInteractionResponseMessage::new().content("You have left the campaign"),
                        ))
                        .into_response(),
                        Err(e) => match e {
                            crate::shared::error::DatabaseError::ConstraintError(c) => match c {
                                crate::shared::error::ConstraintError::RemoveStoryteller => {
                                    invalid_command_message("the storyteller cannot leave the campaign until they designate a new storyteller with **/campaign storyteller set**")
                                }
                                _ => internal_server_error(),
                            },
                            crate::shared::error::DatabaseError::NotFound(missing) => {
                                invalid_command_message(&format!("{} not found", missing))
                            }
                            _ => internal_server_error(),
                        },
                    }
                }
                "leave_campaign_cancel" => Json(CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new().content("Operation cancelled."),
                ))
                .into_response(),
                _ => acknowledge_component(),
            }
        }
        _ => acknowledge_component(),
    }
}
