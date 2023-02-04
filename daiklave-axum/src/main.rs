#![warn(missing_docs)]
//! daiklave-axum is the binary for the HTTP server component of the Daiklave 
//! app. It is responsible for handing Discord webhook interactions; serving
//! static content for the Yew application; and serving API requests from the
//! Yew client (and potentially other 3rd party Exalted tools.)

/// The module responsible for handling all interactions with Discord, both 
/// responding to incoming POSTs and serving outgoing requests their API.
pub mod discord;

use std::net::SocketAddr;

use axum::{
    routing::{get, post}, Router,
};

use crate::discord::{create_app_commands::create_app_commands, post_discord_handler};

/// 
#[derive(Clone)]
pub struct AppState {
    public_key: ed25519_dalek::PublicKey,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    create_app_commands().await;

    // Public key for verifying incoming POST requests from Discord
    let public_key = ed25519_dalek::PublicKey::from_bytes(
        std::env::var("DISCORD_PUBLIC_KEY")
            .expect("Expected DISCORD_PUBLIC_KEY in environment")
            .as_bytes(),
    )
    .expect("Expected DISCORD_PUBLIC_KEY to be a valid ed25519 public key");
    
    // All resources needed to handle a request not contained in the request's 
    // URL, headers, or body.
    let state = AppState { public_key };

    // Initialize the router for the app.
    let app = Router::new()
        .route("/", get(root))
        .route("/discord", post(post_discord_handler))
        .with_state(state);
   

    // Start listening on port 3000
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

