use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serenity::builder::{CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage};

pub fn character_concept_help() -> Response {
    Json(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().embed(
            CreateEmbed::new()
                .title("/character concept")
                .field("delete", "Remove your character concept", false)
                .field(
                    "help",
                    "Shows the available character concept subcommands",
                    false,
                )
                .field(
                    "set",
                    "Add or update your character concept to the specified value",
                    false,
                )
                .field(
                    "show",
                    "Display your character concept (if you have one)",
                    false,
                ),
        ),
    ))
    .into_response()
}
