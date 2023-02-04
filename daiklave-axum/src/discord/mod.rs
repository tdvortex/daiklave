/// The module for the initial definition of app commands in Discord.
pub mod create_app_commands;

/// The module for receiving and responding to POST requests from Discord
/// to the interactions endpoint. Interactions may be one of the following:
/// * Ping: used by Discord to confirm the endpoint is alive
/// * Command: used when a user explicitly invokes a command from their
/// Discord client
/// * Autocomplete: sent while a user is filling out an application command
/// * Component: sent when a user clicks a button or makes a selection from
/// a string select list or contextual select list.
/// * Modal: sent when a user closes a modal popup.
pub mod handle_interaction;

use axum::response::IntoResponse;
use axum::{
    extract::{RawBody, State},
    response::Response,
};
use ed25519_dalek::Verifier;
use handle_interaction::handle_interaction;
use hyper::body::to_bytes;
use hyper::{HeaderMap, StatusCode};
use serenity::all::Interaction;

use crate::AppState;

const DISCORD_API_URL_BASE: &str = "https://discord.com/api/v10/";

/// The handler for POST requests to the Discord endpoint.
pub async fn post_discord_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    RawBody(raw_body): RawBody,
) -> Response {
    let AppState { public_key } = state;

    // Verify discord interaction signature
    let signature: ed25519_dalek::Signature = match headers
        .get("X-Signature-Ed25519")
        .map(|header_value| header_value.as_bytes().try_into())
    {
        Some(Ok(signature)) => signature,
        _ => {
            return (StatusCode::UNAUTHORIZED, "invalid request signature").into_response();
        }
    };

    let timestamp = match headers
        .get("X-Signature-Timestamp")
        .map(|header_value| header_value.as_bytes())
    {
        Some(bytes) => bytes,
        _ => {
            return (StatusCode::UNAUTHORIZED, "invalid request signature").into_response();
        }
    };

    let message_bytes = match to_bytes(raw_body).await {
        Ok(bytes) => bytes,
        Err(_) => {
            return (StatusCode::UNAUTHORIZED, "invalid request signature").into_response();
        }
    };

    let mut joined_message = timestamp.to_vec();
    joined_message.extend_from_slice(&message_bytes);

    if public_key.verify(&joined_message, &signature).is_err() {
        return (StatusCode::UNAUTHORIZED, "invalid request signature").into_response();
    }

    // Deserialize the raw message bytes into a typed Interaction
    let interaction = match serde_json::from_slice::<Interaction>(&message_bytes) {
        Ok(interaction) => interaction,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, ()).into_response();
        }
    };

    // Handle the interaction. This should always return 200/OK.
    handle_interaction(&interaction)
}
