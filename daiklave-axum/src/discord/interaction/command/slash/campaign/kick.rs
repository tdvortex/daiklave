use axum::response::Response;
use serenity::all::CommandInteraction;

use crate::{
    discord::{
        components::kick_player_message,
        get_channel_auth,
        interaction::{forbidden, internal_server_error, invalid_command_message, not_authorized},
        partial::PartialKickPlayer,
        ChannelAuthResult,
    },
    AppState,
};

pub async fn campaign_kick(interaction: &CommandInteraction, state: &mut AppState) -> Response {
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

    let token = interaction.token.clone();

    let kicked_id = if let Some(&id) = interaction.data.resolved.users.keys().next() {
        id
    } else {
        return invalid_command_message("must provide user to kick");
    };

    if kicked_id == user_id {
        return invalid_command_message("the storyteller cannot be kicked");
    }

    if (PartialKickPlayer {
        campaign_id,
        kicked_id,
    })
    .save(token, &mut state.redis_connection_manager)
    .await
    .is_err()
    {
        return internal_server_error();
    }

    kick_player_message(kicked_id)
}
