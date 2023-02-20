use axum::response::Response;
use serenity::all::CommandInteraction;

use crate::{
    discord::{
        components::set_concept_message,
        get_channel_auth,
        interaction::{internal_server_error, no_active_character, not_authorized},
        ChannelAuthResult,
    },
    AppState,
};

pub async fn character_concept_set(
    interaction: &CommandInteraction,
    state: &mut AppState,
) -> Response {
    let user_id = interaction.user.id;
    let channel_id = interaction.channel_id;
    let active_character = match get_channel_auth(state, user_id, channel_id).await {
        Ok(ChannelAuthResult::NotInCampaign) => {
            return not_authorized();
        }
        Ok(ChannelAuthResult::Player {
            campaign_id: _,
            active_character,
        })
        | Ok(ChannelAuthResult::Storyteller {
            campaign_id: _,
            active_character,
        }) => active_character,
        Err(_) => {
            return internal_server_error();
        }
    };

    if active_character.is_none() {
        return no_active_character();
    }

    set_concept_message()
}
