use axum::response::Response;
use serenity::all::CommandInteraction;

use crate::{
    discord::{
        components::leave_campaign_message,
        get_channel_auth,
        interaction::{internal_server_error, invalid_command_message, not_authorized},
        partial::PartialLeaveCampaign,
        ChannelAuthResult,
    },
    AppState,
};

pub async fn campaign_leave(interaction: &CommandInteraction, state: &mut AppState) -> Response {
    let user_id = interaction.user.id;
    let channel_id = interaction.channel_id;
    let campaign_id = match get_channel_auth(state, user_id, channel_id).await {
        Ok(ChannelAuthResult::NotInCampaign) => {
            return not_authorized();
        }
        Ok(ChannelAuthResult::Player {
            campaign_id,
            active_character: _,
        }) => campaign_id,
        Ok(ChannelAuthResult::Storyteller {
            campaign_id,
            active_character: _,
        }) => campaign_id,
        Err(_) => {
            return invalid_command_message("the storyteller cannot leave the campaign until they designate a new storyteller with **/campaign storyteller set**");
        }
    };

    let token = interaction.token.clone();

    if (PartialLeaveCampaign {
        campaign_id,
        user_id,
    })
    .save(token, &mut state.redis_connection_manager)
    .await
    .is_err()
    {
        return internal_server_error();
    }

    leave_campaign_message()
}
