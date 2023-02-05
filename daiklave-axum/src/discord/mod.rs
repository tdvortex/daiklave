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

/// The handler for POST requests to the Discord endpoint.
pub async fn post_discord_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    RawBody(raw_body): RawBody,
) -> Response {
    let AppState { public_key } = state;

    // Verify discord interaction signature
    // Check that "X-Signature-Ed25519" header is present
    let header_value = if let Some(header_value) = headers.get("X-Signature-Ed25519") {
        header_value
    } else {
        return (StatusCode::UNAUTHORIZED, "invalid request signature").into_response();
    };
    // Check that it is a value utf-8 string
    let header_str = if let Ok(s) = header_value.to_str() {
        s
    } else {
        return (StatusCode::UNAUTHORIZED, "invalid request signature").into_response();
    };
    // Check that the utf-8 string decodes to a byte vec
    let bytes = if let Ok(v) = hex::decode(header_str) {
        v
    } else {
        return (StatusCode::UNAUTHORIZED, "invalid request signature").into_response();
    };
    // Check that the bytes are a valid ed25519 signature
    let signature = if let Ok(sig) = ed25519_dalek::Signature::from_bytes(&bytes) {
        sig
    } else {
        return (StatusCode::UNAUTHORIZED, "invalid request signature").into_response();
    };

    // Check that "X-Signature-Timestamp" header is present
    let header_value = if let Some(header_value) = headers.get("X-Signature-Timestamp") {
        header_value
    } else {
        return (StatusCode::UNAUTHORIZED, "invalid request signature").into_response();
    };
    // Check that it is a value utf-8 string
    let timestamp_str = if let Ok(s) = header_value.to_str() {
        s
    } else {
        return (StatusCode::UNAUTHORIZED, "invalid request signature").into_response();
    };

    // Get the body of the message as bytes
    let bytes = if let Ok(b) = to_bytes(raw_body).await {
        b
    } else {
        return (StatusCode::UNAUTHORIZED, "invalid request signature").into_response();
    };
    // Get the body of the message as a str
    let body_str = if let Ok(s) = std::str::from_utf8(&bytes) {
        s
    } else {
        return (StatusCode::UNAUTHORIZED, "invalid request signature").into_response();
    };

    // Concatenate the timestamp and the body into a single UTF-8 string
    let joined_string = format!("{}{}", timestamp_str, body_str);

    // Convert the joined utf-8 string back to bytes and verify it using the 
    // public key and signature
    if public_key.verify(joined_string.as_bytes(), &signature).is_err() {
        return (StatusCode::UNAUTHORIZED, "invalid request signature").into_response();
    }

    // Deserialize the raw message string into a typed Interaction
    let interaction = match serde_json::from_str::<Interaction>(body_str) {
        Ok(interaction) => interaction,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, ()).into_response();
        }
    };

    // Handle the interaction. This should always return 200/OK.
    // This is isolated for readability and testability.
    handle_interaction(&interaction)
}