use axum::{
    response::{IntoResponse, Response},
    Json,
};
use daiklave_core::{mutations::SetConcept, CharacterMutation};
use serenity::{
    all::{ActionRowComponent, ModalInteraction},
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
};

use crate::{
    discord::{
        get_channel_auth,
        interaction::{internal_server_error, no_active_character, not_authorized},
        ChannelAuthResult,
    },
    shared::character::PatchCharacter,
    AppState,
};

/// Handle the closing of a set concept modal by changing the character's
/// concept as specified in the modal's text box.
pub async fn set_concept_modal(interaction: &ModalInteraction, state: &mut AppState) -> Response {
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

    let Some(action_row) = interaction.data.components.iter().next() else {
        return internal_server_error();
    };

    let Some(ActionRowComponent::InputText(input_text)) = action_row.components.iter().next() else {
        return internal_server_error();
    };

    if input_text.custom_id != "set_concept" {
        return internal_server_error();
    }

    let mutation = CharacterMutation::SetConcept(SetConcept(input_text.value.clone()));

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

    if patch_result.is_ok() {
        Json(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(format!(
                "Character name successfully changed to {}",
                input_text.value
            )),
        ))
        .into_response()
    } else {
        internal_server_error()
    }
}
