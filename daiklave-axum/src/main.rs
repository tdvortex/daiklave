#![warn(missing_docs)]
//! daiklave-axum is the binary for the HTTP server component of the Daiklave
//! app. It is responsible for handing Discord webhook interactions; serving
//! static content for the Yew application; and serving API requests from the
//! Yew client (and potentially other 3rd party Exalted tools.)

/// The module responsible for handling all interactions with Discord, both
/// responding to incoming POSTs and serving outgoing requests their API.
pub mod discord;

use std::net::SocketAddr;

use axum::{routing::post, Router};
use axum_extra::routing::SpaRouter;

use crate::discord::post_discord_handler;
use hex::decode;

/// Any handles or resources not tied to an individual request.
#[derive(Clone)]
pub struct AppState {
    discord_public_key: ed25519_dalek::PublicKey,
    _mongodb_client: mongodb::Client,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Public key for verifying incoming POST requests from Discord
    let hex_string =
        std::env::var("DISCORD_PUBLIC_KEY").expect("Expected DISCORD_PUBLIC_KEY in environment");
    let hex_bytes =
        decode(hex_string).expect("Expected DISCORD_PUBLIC_KEY to be valid hexadecimal");

    let discord_public_key = ed25519_dalek::PublicKey::from_bytes(&hex_bytes)
        .expect("Expected DISCORD_PUBLIC_KEY to be a valid ed25519 public key");

    // Handle to connect to mongodb
    let mongodb_username =
        std::env::var("MONGDOB_USER").expect("Expected MONGODB_USER in environment");
    let mongodb_password =
        std::env::var("MONGODB_PASSWORD").expect("Expected MONGODB_PASSWORD in environment");
    let mongodb_url = std::env::var("MONGODB_URL").expect("Expected MONGODB_URL in environment");
    let mongodb_connection_string = format!(
        "mongodb+srv://{}:{}@{}/?retryWrites=true&w=majority",
        mongodb_username, mongodb_password, mongodb_url
    );
    let mongodb_client = mongodb::Client::with_options(
        mongodb::options::ClientOptions::parse(mongodb_connection_string)
            .await
            .expect("Expected successful connection to MongdoDB"),
    )
    .expect("Expected successful connection to MongdoDB");

    // All resources needed to handle a request not contained in the request's
    // URL, headers, or body.
    let state = AppState { 
        discord_public_key,
        _mongodb_client: mongodb_client,
    };

    // Initialize the router for the app.
    let app = Router::new()
        .merge(SpaRouter::new("/assets", "assets"))
        .route("/discord", post(post_discord_handler))
        .with_state(state);

    // Start listening on port 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("unexpected error");
}
