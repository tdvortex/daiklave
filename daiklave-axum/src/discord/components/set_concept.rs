use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serenity::{
    all::InputTextStyle,
    builder::{CreateActionRow, CreateInputText, CreateInteractionResponse, CreateModal},
};

/// Creates a modal popup with a single field to enter a new character concept.
pub fn set_concept_message() -> Response {
    let input_text = CreateActionRow::InputText(
        CreateInputText::new(InputTextStyle::Short, "Concept", "set_concept_field")
            .placeholder("Enter your character concept here")
            .min_length(1)
            .max_length(255)
            .required(true),
    );

    Json(CreateInteractionResponse::Modal(
        CreateModal::new("set_concept", "Set Concept").components(vec![input_text]),
    ))
    .into_response()
}
