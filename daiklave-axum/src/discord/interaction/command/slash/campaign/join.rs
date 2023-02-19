use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serenity::{
    all::CommandInteraction,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
};

use crate::{
    discord::interaction::{internal_server_error, invalid_command_message},
    shared::{campaign::AddCampaignPlayer, error::DatabaseError},
    AppState,
};

pub async fn campaign_join(interaction: &CommandInteraction, state: &mut AppState) -> Response {
    let user_id = interaction.user.id;
    let channel_id = interaction.channel_id;

    let database = state.mongodb_client.database(&state.mongodb_database_name);
    let mut session = if let Ok(session) = state.mongodb_client.start_session(None).await {
        session
    } else {
        return internal_server_error();
    };

    let join_result = (AddCampaignPlayer {
        channel_id,
        user_id,
    })
    .execute(&database, &mut session)
    .await;

    match join_result {
        Ok(campaign_name) => Json(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(format!(
                "Welcome to {}, <@{}>!",
                campaign_name,
                user_id.0.get()
            )),
        ))
        .into_response(),
        Err(e) => match e {
            DatabaseError::NotFound(_) => invalid_command_message("no campaign for this channel"),
            _ => internal_server_error(),
        },
    }
}
