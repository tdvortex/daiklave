use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serenity::{
    all::{ButtonStyle, UserId},
    builder::{CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage},
};

/// Creates a message component interaction with two buttons:
/// Button 1: a red button saying "Kick player"
/// Button 2: a grey button saying "Cancel"
pub fn kick_player_message(kicked_id: UserId) -> Response {
    Json(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content(format!("Are you sure you want to kick player <@{}>? This will delete all of their characters in this campaign!", kicked_id.0.get()))
            .button(CreateButton::new("kick_player_confirm").label("Kick player").style(ButtonStyle::Danger))
            .button(CreateButton::new("kick_player_cancel").label("Cancel").style(ButtonStyle::Secondary))
    ))
    .into_response()
}
