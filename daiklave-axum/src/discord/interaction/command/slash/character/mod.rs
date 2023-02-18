mod concept;
mod create;
mod delete;
mod help;
mod rename;
mod switch;

use axum::response::Response;
use serenity::all::CommandInteraction;

use crate::{discord::interaction::unknown_command_message, AppState};

use self::{
    concept::character_concept, create::character_create, delete::character_delete,
    help::character_help, rename::character_rename, switch::character_switch,
};

pub async fn character(interaction: &CommandInteraction, state: &mut AppState) -> Response {
    if let Some(option) = interaction.data.options.first() {
        match option.name.as_str() {
            "concept" => character_concept(interaction, state).await,
            "create" => character_create(interaction, state).await,
            "delete" => character_delete(interaction, state).await,
            "help" => character_help(),
            "rename" => character_rename(interaction, state).await,
            "switch" => character_switch(interaction, state).await,
            other => unknown_command_message(&format!("character {}", other)),
        }
    } else {
        unknown_command_message(interaction.data.name.as_str())
    }
}
