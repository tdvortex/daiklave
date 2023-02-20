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
        partial::PartialKickPlayer,
    },
    shared::campaign::RemoveCampaignPlayer,
    AppState,
};

pub async fn kick_player_components(
    component_interaction: &ComponentInteraction,
    state: &mut AppState,
) -> Response {
    // Load whatever we have saved in Redis for this interaction
    let old_partial: PartialKickPlayer = match PartialKickPlayer::load(
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
                "kick_player_confirm" => {
                    let database = state.mongodb_client.database(&state.mongodb_database_name);
                    let mut session =
                        if let Ok(session) = state.mongodb_client.start_session(None).await {
                            session
                        } else {
                            return internal_server_error();
                        };

                    let remove_result = (RemoveCampaignPlayer {
                        campaign_id: old_partial.campaign_id,
                        user_id: old_partial.kicked_id,
                    })
                    .execute(&database, &mut session, &mut state.redis_connection_manager)
                    .await;

                    match remove_result {
                        Ok(_) => Json(CreateInteractionResponse::UpdateMessage(
                            CreateInteractionResponseMessage::new().content(format!(
                                "<@{}> has been kicked",
                                old_partial.kicked_id.0.get()
                            )),
                        ))
                        .into_response(),
                        Err(e) => match e {
                            crate::shared::error::DatabaseError::ConstraintError(c) => match c {
                                crate::shared::error::ConstraintError::RemoveStoryteller => {
                                    invalid_command_message("the storyteller cannot be kicked")
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
                "kick_player_cancel" => Json(CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new().content("Operation cancelled."),
                ))
                .into_response(),
                _ => acknowledge_component(),
            }
        }
        _ => acknowledge_component(),
    }
}
