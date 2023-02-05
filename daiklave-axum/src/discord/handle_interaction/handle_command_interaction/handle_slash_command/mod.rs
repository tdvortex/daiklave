use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serenity::{
    all::CommandInteraction,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
};

use crate::discord::handle_interaction::unknown_command_message;

/// Handle a slash command (of type CHAT INPUT).
pub fn handle_slash_command(interaction: &CommandInteraction) -> Response {
    // Exact data needs may vary by command, but we need the name to route it
    let command_name = interaction.data.name.as_str();

    match command_name {
        "version" => Json(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content("Daiklave version 0.1.0"),
        ))
        .into_response(),
        // We don't have support for this command yet
        other_name => unknown_command_message(other_name),
    }
}

mod test {
    #[test]
    fn test_version() {
        use hyper::StatusCode;
        use serenity::all::{Interaction};
        use crate::discord::handle_interaction::handle_interaction;
        let interaction_json = serde_json::json!({
            "id": 123456,
            "application_id": 123456,
            "type": 2,
            "data": {
                "id": 123456,
                "name": "version",
                "type": 1,
                "resolved": {
                    "users": {},
                    "members": {},
                    "roles": {},
                    "channels": {},
                    "messages": {},
                    "attachments": {},
                },
                "options": [],
            },
            "channel_id": 123456,
            "token": "continuation",
            "version": 1,
            "locale": "en-US",
        });

        let interaction_string = interaction_json.to_string();

        let interaction = serde_json::from_str::<Interaction>(&interaction_string).unwrap();

        let response = handle_interaction(&interaction);
        assert_eq!(response.status(), StatusCode::OK)
    }
}
