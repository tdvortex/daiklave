use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serenity::builder::{CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage};

pub fn campaign_channels_help() -> Response {
    Json(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().embed(
            CreateEmbed::new()
                .title("/campaign channels")
                .field("help", "Shows the available channel subcommands", false)
                .field("set", "Sets the channels used by this campaign", false)
                .field("show", "Shows the channels used by this campaign", false),
        ),
    ))
    .into_response()
}
