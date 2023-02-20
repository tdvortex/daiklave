use axum::{response::{Response, IntoResponse}, Json};
use serenity::{all::CommandInteraction, builder::{CreateInteractionResponse, CreateInteractionResponseMessage}};

use crate::{AppState, discord::{get_channel_auth, interaction::{internal_server_error, not_authorized, no_active_character}, ChannelAuthResult}, shared::character::GetCharacter};

pub async fn character_concept_show(
    interaction: &CommandInteraction,
    state: &mut AppState,
) -> Response {
    let user_id = interaction.user.id;
    let channel_id = interaction.channel_id;
    let (campaign_id, maybe_active_character) = match get_channel_auth(state, user_id, channel_id).await {
        Ok(ChannelAuthResult::NotInCampaign) => {return not_authorized();}
        Ok(ChannelAuthResult::Player { campaign_id, active_character }) | Ok(ChannelAuthResult::Storyteller { campaign_id, active_character }) => (campaign_id, active_character),
        Err(_) => {return internal_server_error();}
    };

    let Some(character_id) = maybe_active_character else {
        return no_active_character();
    };
    let database = state.mongodb_client.database(&state.mongodb_database_name);

    let Ok(Some(document)) = (GetCharacter {
        player: user_id,
        campaign_id,
        character_id,
    }).execute(&database, &mut state.redis_connection_manager).await else {
        return internal_server_error();
    };

    let character: daiklave_core::Character = (&document.character).into();

    let content = if let Some(concept) = character.concept() {
        format!("{}: {}", character.name(), concept)
    } else {
        format!("{} has no character concept. Use **/character concept set** to add one.", character.name())
    };

    Json(
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(content)
        )
    ).into_response()
}
