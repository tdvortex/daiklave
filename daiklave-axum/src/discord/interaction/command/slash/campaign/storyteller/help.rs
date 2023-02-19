use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serenity::builder::{CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage};

pub fn campaign_storyteller_help() -> Response {
    Json(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
        .embed(
            CreateEmbed::new()
            .title("/campaign storyteller")
            .field("help", "Shows the available storyteller subcommands", false)
            .field("set", "Reassigns the storyteller for the campaign. Only the storyteller can use this command.", false)
        )
    )).into_response()
}
