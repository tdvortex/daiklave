use std::collections::HashSet;

use axum::response::Response;
use serenity::all::CommandInteraction;

use crate::{
    discord::{
        components::set_channels_message,
        get_channel_auth,
        interaction::{forbidden, internal_server_error, not_authorized},
        partial::PartialSetChannels,
        ChannelAuthResult,
    },
    AppState,
};

pub async fn campaign_channels_set(
    interaction: &CommandInteraction,
    state: &mut AppState,
) -> Response {
    let token = interaction.token.clone();
    let user_id = interaction.user.id;
    let channel_id = interaction.channel_id;

    let campaign_id = match get_channel_auth(state, user_id, channel_id).await {
        Ok(ChannelAuthResult::NotInCampaign) => {
            return not_authorized();
        }
        Ok(ChannelAuthResult::Player {
            campaign_id: _,
            active_character: _,
        }) => {
            return forbidden();
        }
        Ok(ChannelAuthResult::Storyteller {
            campaign_id,
            active_character: _,
        }) => campaign_id,
        Err(_) => {
            return internal_server_error();
        }
    };

    let partial_set_channels = PartialSetChannels {
        dice_channel: None,
        channels: HashSet::new(),
        campaign_id,
    };

    if partial_set_channels
        .save(token, &mut state.redis_connection_manager)
        .await
        .is_err()
    {
        return internal_server_error();
    }

    set_channels_message(false)
}
