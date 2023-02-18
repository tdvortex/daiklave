mod delete;
mod help;
mod set;
mod show;

use axum::response::Response;
use serenity::all::{CommandDataOptionValue, CommandInteraction};

use crate::{discord::interaction::unknown_command_message, AppState};

use self::{
    delete::character_concept_delete, help::character_concept_help, set::character_concept_set,
    show::character_concept_show,
};

pub async fn character_concept(interaction: &CommandInteraction, state: &mut AppState) -> Response {
    let character_concept = if let Some(option) = interaction.data.options.first() {
        match option.name.as_str() {
            "concept" => &option.value,
            other => {
                return unknown_command_message(&format!("character {}", other));
            }
        }
    } else {
        return unknown_command_message(interaction.data.name.as_str());
    };

    if let Some(concept_subcommand) = match character_concept {
        CommandDataOptionValue::SubCommandGroup(concept_group) => concept_group.first(),
        _ => {
            return unknown_command_message(interaction.data.name.as_str());
        }
    } {
        match concept_subcommand.name.as_str() {
            "delete" => character_concept_delete(interaction, state).await,
            "help" => character_concept_help(),
            "set" => character_concept_set(interaction, state).await,
            "show" => character_concept_show(interaction, state).await,
            other => unknown_command_message(&format!("character concept {}", other)),
        }
    } else {
        unknown_command_message("campaign concept")
    }
}
