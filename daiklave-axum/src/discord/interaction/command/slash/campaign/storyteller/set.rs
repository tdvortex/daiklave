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
        interaction::{forbidden, internal_server_error, invalid_command_message, not_authorized},
        ChannelAuthResult,
    },
    shared::{
        campaign::SetCampaignStoryteller,
        error::{ConstraintError, DatabaseError},
    },
    AppState,
};

pub async fn campaign_storyteller_set(
    interaction: &CommandInteraction,
    state: &mut AppState,
) -> Response {
    let user_id = interaction.user.id;
    let channel_id = interaction.channel_id;
    let auth_result = get_channel_auth(state, user_id, channel_id).await;

    let campaign_id = match auth_result {
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

    let new_storyteller = if let Some(&user_id) = interaction.data.resolved.users.keys().next() {
        user_id
    } else {
        return invalid_command_message("new storyteller is required");
    };

    let database = state.mongodb_client.database(&state.mongodb_database_name);
    let mut session = if let Ok(session) = state.mongodb_client.start_session(None).await {
        session
    } else {
        return internal_server_error();
    };

    let update_result = SetCampaignStoryteller {
        campaign_id,
        old_storyteller: user_id,
        new_storyteller,
    }
    .execute(&database, &mut session, &mut state.redis_connection_manager)
    .await;

    match update_result {
        Ok(_) => Json(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::default().content(format!(
                "<@{}> is now the storyteller.",
                new_storyteller.0.get()
            )),
        ))
        .into_response(),
        Err(e) => match e {
            DatabaseError::ConstraintError(c) => match c {
                ConstraintError::StorytellerNotPlayer => invalid_command_message(&format!(
                    "<@{}> is not a player.\n Use **/campaign join** first, then try again.",
                    new_storyteller.0.get()
                )),
                _ => internal_server_error(),
            },
            DatabaseError::NotFound(_) => invalid_command_message("campaign not found"),
            _ => internal_server_error(),
        },
    }
}
