use axum::{response::{Response, IntoResponse}, Json};
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage, CreateEmbed};

pub fn character_help() -> Response {
    Json(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
        .embed(
            CreateEmbed::new()
            .title("/character")
            .field("concept", "Subcommands related to your character concept", false)
            .field("create", "Create a new character", false)
            .field("delete", "Delete this character forever", false)
            .field("help", "Shows the available character subcommands", false)
            .field("rename", "Changes your character's name", false)
            .field("switch", "Switch your active character for this campaign", false)
        )
    )).into_response()
}
