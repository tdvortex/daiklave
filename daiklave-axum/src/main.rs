use std::net::SocketAddr;

use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router, extract::{RawBody, State},
};
use hyper::body::to_bytes;
use ed25519_dalek::Verifier;

use serenity::{all::Interaction, builder::CreateInteractionResponse};

#[derive(Clone)]
struct AppState {
    public_key: ed25519_dalek::PublicKey,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let public_key = ed25519_dalek::PublicKey::from_bytes(
        std::env::var("DISCORD_PUBLIC_KEY")
            .expect("Discord public key in environment")
            .as_bytes(),
    )
    .expect("invalid public key");
    let state = AppState { public_key };

    let app = Router::new()
        .route("/", get(root))
        .route("/discord", post(discord_interaction))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("unexpected error");
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn discord_interaction(State(state): State<AppState>, headers: HeaderMap, RawBody(raw_body): RawBody) -> Response {
    let AppState {
        public_key
    } = state;

    // Verify discord interaction signature
    let signature: ed25519_dalek::Signature = match headers.get("X-Signature-Ed25519").map(|header_value| header_value.as_bytes().try_into()) {
        Some(Ok(signature)) => signature,
        _ => {return (StatusCode::UNAUTHORIZED, "invalid request signature").into_response();}
    };

    let timestamp = match headers.get("X-Signature-Timestamp").map(|header_value| header_value.as_bytes()) {
        Some(bytes) => bytes,
        _ => {return (StatusCode::UNAUTHORIZED, "invalid request signature").into_response();}
    };

    let message_bytes = match to_bytes(raw_body).await {
        Ok(bytes) => bytes,
        Err(_) => {return (StatusCode::UNAUTHORIZED, "invalid request signature").into_response();}
    };

    let mut joined_message = timestamp.to_vec();
    joined_message.extend_from_slice(&message_bytes);

    if public_key.verify(&joined_message, &signature).is_err() {
        return (StatusCode::UNAUTHORIZED, "invalid request signature").into_response();
    }

    // Deserialize the raw message bytes into a typed Interaction
    let interaction = match serde_json::from_slice::<Interaction>(&message_bytes) {
        Ok(interaction) => interaction,
        Err(_) => {return (StatusCode::BAD_REQUEST, ()).into_response();}
    };

    // Handle the interaction
    match interaction {
        Interaction::Ping(_) => Json(CreateInteractionResponse::Pong).into_response(),
        Interaction::Command(_) => ().into_response(),
        Interaction::Autocomplete(_) => ().into_response(),
        Interaction::Component(_) => ().into_response(),
        Interaction::Modal(_) => ().into_response(),
    }
}

