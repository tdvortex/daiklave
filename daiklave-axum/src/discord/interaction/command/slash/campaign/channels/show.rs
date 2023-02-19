use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serenity::{
    all::CommandInteraction,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
};

use crate::{
    discord::{
        get_channel_auth,
        interaction::{internal_server_error, not_authorized},
        ChannelAuthResult,
    },
    shared::{campaign::GetCampaign},
    AppState,
};

pub async fn campaign_channels_show(
    interaction: &CommandInteraction,
    state: &mut AppState,
) -> Response {
    let user_id = interaction.user.id;
    let channel_id = interaction.channel_id;

    let campaign_id = match get_channel_auth(state, user_id, channel_id).await {
        Ok(ChannelAuthResult::NotInCampaign) => {
            return not_authorized();
        }
        Err(_) => {
            return internal_server_error();
        }
        Ok(ChannelAuthResult::Player {
            campaign_id,
            active_character: _,
        })
        | Ok(ChannelAuthResult::Storyteller {
            campaign_id,
            active_character: _,
        }) => campaign_id,
    };

    let get_campaign_result = GetCampaign {
        user_id,
        campaign_id,
    }
    .execute(&state.mongodb_client.database(&state.mongodb_database_name))
    .await;

    match get_campaign_result {
        Ok(None) => not_authorized(),
        Ok(Some(campaign)) => {
            let mut response_text = format!("**Channels for {}**:\n", campaign.name);
            response_text.push_str(&format!(
                "Dice channel: <#{}>\n",
                campaign.dice_channel.0.get()
            ));
            response_text.push_str("Other channels:\n");
            for &channel in campaign
                .channels
                .iter()
                .filter(|channel_id| channel_id.0 != campaign.dice_channel.0)
            {
                response_text.push_str(&format!("<#{}>\n", channel.0.get()));
            }

            Json(CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::default().content(response_text),
            ))
            .into_response()
        }
        Err(_) => internal_server_error(),
    }
}
