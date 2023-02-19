use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serenity::builder::{CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage};

pub fn help() -> Response {
    Json(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().embed(
            CreateEmbed::new()
                .title("Daiklave slash commands")
                .field(
                    "/campaign",
                    "Commands related to campaign management",
                    false,
                )
                .field(
                    "/character",
                    "Commands related to general character management",
                    false,
                )
                .field("/help", "View the available Daiklave slash commands", false)
                .field("/vesion", "Shows the current version of Daiklave", false),
        ),
    ))
    .into_response()
}
