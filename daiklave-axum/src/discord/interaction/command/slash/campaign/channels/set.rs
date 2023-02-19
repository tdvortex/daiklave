use std::collections::HashSet;

use axum::response::Response;
use serenity::all::CommandInteraction;

use crate::{
    discord::{
        components::set_channels_message,
        interaction::{forbidden, internal_server_error, not_authorized},
        partial::PartialSetChannels,
    },
    shared::authorization::GetChannelAuthorization,
    AppState,
};

pub async fn campaign_channels_set(
    interaction: &CommandInteraction,
    state: &mut AppState,
) -> Response {
    let token = interaction.token.clone();
    let user_id = interaction.user.id;
    let channel_id = interaction.channel_id;
    let database = state.mongodb_client.database(&state.mongodb_database_name);
    let connection = &mut state.redis_connection_manager;

    let campaign_id = if let Ok(maybe_auth) = (GetChannelAuthorization {
        user_id,
        channel_id,
    })
    .execute(&database, connection)
    .await
    {
        if let Some(auth) = maybe_auth {
            if auth.is_storyteller {
                auth.campaign_id
            } else {
                return forbidden();
            }
        } else {
            return not_authorized();
        }
    } else {
        return internal_server_error();
    };

    let partial_set_channels = PartialSetChannels {
        dice_channel: None,
        channels: HashSet::new(),
        campaign_id,
    };

    if partial_set_channels
        .save_partial(token, connection)
        .await
        .is_err()
    {
        return internal_server_error();
    }

    set_channels_message(false)
}
