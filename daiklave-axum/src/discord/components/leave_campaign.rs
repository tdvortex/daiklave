use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serenity::{
    all::ButtonStyle,
    builder::{CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage},
};

/// Creates a message component interaction with two buttons:
/// Button 1: a red button saying "Leave campaign"
/// Button 2: a grey button saying "Cancel"
pub fn leave_campaign_message() -> Response {
    Json(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content("Are you sure you want to leave this campaign? This will delete all of your characters in this campaign!")
            .button(CreateButton::new("leave_campaign_confirm").label("Leave campaign").style(ButtonStyle::Danger))
            .button(CreateButton::new("leave_campaign_cancel").label("Cancel").style(ButtonStyle::Secondary))
    ))
    .into_response()
}
