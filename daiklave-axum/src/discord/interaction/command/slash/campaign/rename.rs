use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serenity::{
    all::{CommandDataOptionValue, CommandInteraction},
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
};

use crate::{
    discord::{
        get_channel_auth,
        interaction::{forbidden, internal_server_error, invalid_command_message, not_authorized},
        ChannelAuthResult,
    },
    shared::{campaign::RenameCampaign, error::DatabaseError},
    AppState,
};

pub async fn campaign_rename(interaction: &CommandInteraction, state: &mut AppState) -> Response {
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

    let subcommand = if let Some(subcommand) = interaction.data.options.first() {
        subcommand
    } else {
        return invalid_command_message("/campaign requires a subcommand");
    };

    let params = match &subcommand.value {
        CommandDataOptionValue::SubCommand(params) => params,
        _ => {
            return invalid_command_message("/campaign rename should be a subcommand");
        }
    };

    let name = if let Some(name_option) = params.iter().find(|option| option.name == "name") {
        match &name_option.value {
            CommandDataOptionValue::String(name) => name.to_owned(),
            _ => {
                return invalid_command_message("Campaign name must be a string");
            }
        }
    } else {
        return invalid_command_message("Campaign name is required");
    };

    let database = state.mongodb_client.database(&state.mongodb_database_name);
    let mut session = if let Ok(session) = state.mongodb_client.start_session(None).await {
        session
    } else {
        return internal_server_error();
    };

    let rename_result = (RenameCampaign {
        campaign_id,
        name: name.clone(),
    })
    .execute(&database, &mut session)
    .await;

    match rename_result {
        Ok(_) => Json(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content(format!("Campaign successfully renamed to \"{}\"", name)),
        ))
        .into_response(),
        Err(e) => match e {
            DatabaseError::NotFound(missing) => {
                invalid_command_message(&format!("not found: {}", missing))
            }
            _ => internal_server_error(),
        },
    }
}
