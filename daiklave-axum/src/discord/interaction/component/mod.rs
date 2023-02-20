mod create_campaign;
mod kick_player;
mod leave_campaign;
mod set_channels;
use create_campaign::create_campaign_components;
use kick_player::kick_player_components;
use leave_campaign::leave_campaign_components;
use set_channels::set_channels_components;

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serenity::{all::ComponentInteraction, builder::CreateInteractionResponse};

use crate::AppState;

/// Creates a 200 OK response with a response to a component or modal interaction with
/// DEFERRED_UPDATE_MESSAGE to acknowledge the interaction but otherwise do nothing.
pub fn acknowledge_component() -> Response {
    Json(CreateInteractionResponse::Acknowledge).into_response()
}

/// Handle an interaction on a message component. This may be a button click
/// or an interaction with a select menu. It does NOT include text fields; text
/// fields only appear on modals.
pub async fn post_component(
    component_interaction: &ComponentInteraction,
    state: &mut AppState,
) -> Response {
    match component_interaction.data.custom_id.as_str() {
        "create_dice_channel" | "create_all_channels" | "create_campaign_submit" => {
            create_campaign_components(component_interaction, state).await
        }
        "set_dice_channel" | "set_all_channels" | "set_channels_submit" => {
            set_channels_components(component_interaction, state).await
        }
        "kick_player_confirm" | "kick_player_cancel" => {
            kick_player_components(component_interaction, state).await
        }
        "leave_campaign_confirm" | "leave_campaign_cancel" => {
            leave_campaign_components(component_interaction, state).await
        }
        // If we get an unexpected component interaction, can use
        // DEFERRED_UPDATE_MESSAGE to tell Discord "acknowledged, do nothing"
        _ => acknowledge_component(),
    }
}
