use axum::{
    response::{IntoResponse, Response},
    Json,
};
use daiklave_core::{CharacterMutation, CharacterMutationError, ConceptError};
use serenity::{
    all::CommandInteraction,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
};

use crate::{
    discord::{
        get_channel_auth,
        interaction::{
            internal_server_error, invalid_command_message, no_active_character, not_authorized,
        },
        ChannelAuthResult,
    },
    shared::{
        character::PatchCharacter,
        error::{ConstraintError, DatabaseError},
    },
    AppState,
};

pub async fn character_concept_delete(
    interaction: &CommandInteraction,
    state: &mut AppState,
) -> Response {
    let user_id = interaction.user.id;
    let channel_id = interaction.channel_id;
    let (campaign_id, maybe_active_character) =
        match get_channel_auth(state, user_id, channel_id).await {
            Ok(ChannelAuthResult::NotInCampaign) => {
                return not_authorized();
            }
            Ok(ChannelAuthResult::Player {
                campaign_id,
                active_character,
            })
            | Ok(ChannelAuthResult::Storyteller {
                campaign_id,
                active_character,
            }) => (campaign_id, active_character),
            Err(_) => {
                return internal_server_error();
            }
        };

    let Some(character_id) = maybe_active_character else {
        return no_active_character();
    };

    let mutation = CharacterMutation::RemoveConcept;

    let patch_result = PatchCharacter {
        player: user_id,
        campaign_id,
        character_id,
        mutation,
    }
    .execute(
        &state.mongodb_client,
        &state.mongodb_database_name,
        &mut state.redis_connection_manager,
    )
    .await;

    match patch_result {
        Ok(_) => Json(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content("Character concept has been deleted."),
        ))
        .into_response(),
        Err(e) => match e {
            DatabaseError::ConstraintError(ConstraintError::MutationError(
                CharacterMutationError::ConceptError(c),
            )) => match c {
                ConceptError::NotFound => {
                    invalid_command_message("character does not have a concept to remove")
                }
            },
            DatabaseError::NotFound(missing) => {
                invalid_command_message(&format!("not found: {}", missing))
            }
            _ => internal_server_error(),
        },
    }
}
