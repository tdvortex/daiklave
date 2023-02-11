#![warn(missing_docs)]
//! daiklave-axum is the binary for the HTTP server component of the Daiklave
//! app. It is responsible for handing Discord webhook interactions; serving
//! static content for the Yew application; and serving API requests from the
//! Yew client (and potentially other 3rd party Exalted tools.)

/// Routes and handlers related to login and session management.
pub mod auth;

/// The module responsible for handling all interactions with Discord, both
/// responding to incoming POSTs and serving outgoing requests their API.
pub mod discord;

use std::net::SocketAddr;

use axum::{routing::{post, get}, Router, extract::FromRef};
use axum_extra::routing::SpaRouter;

use crate::{discord::post_discord_handler, auth::{handle_login, handle_login_callback}};
use hex::decode;
/// Any handles or resources not tied to an individual request.
#[derive(Clone)]
pub struct AppState {
    /// Public key for verifying incoming POST requests from Discord
    pub discord_public_key: ed25519_dalek::PublicKey,
    /// Discord Id for this application
    pub discord_token: String,
    /// Key for signing authentication cookies
    pub cookie_signing_key: axum_extra::extract::cookie::Key,
    /// Handle to connect to mongodb
    pub _mongodb_client: mongodb::Client,
    /// Handle to connect to redis
    pub _redis_client: redis::Client,
}

impl FromRef<AppState> for axum_extra::extract::cookie::Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_signing_key.clone()
    }
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

    // Discord Id for this application
    let discord_token = std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in environment");

    // Key for signing authentication cookies
    let hex_string = std::env::var("COOKIE_SIGNING_KEY").expect("Expected COOKIE_SIGNING_KEY in environment");
    let hex_bytes = decode(hex_string).expect("Expected COOKIE_SIGNING_KEY to be valid hexadecimal");
    let cookie_signing_key = axum_extra::extract::cookie::Key::from(&hex_bytes);

    // Handle to connect to mongodb
    let mongodb_username =
        std::env::var("MONGDOB_USER").expect("Expected MONGODB_USER in environment");
    let mongodb_password =
        std::env::var("MONGODB_PASSWORD").expect("Expected MONGODB_PASSWORD in environment");
    let mongodb_host = std::env::var("MONGODB_URL").expect("Expected MONGODB_URL in environment");
    let mongodb_url = format!(
        "mongodb+srv://{}:{}@{}/?retryWrites=true&w=majority",
        mongodb_username, mongodb_password, mongodb_host
    );
    let mongodb_client = mongodb::Client::with_options(
        mongodb::options::ClientOptions::parse(mongodb_url)
            .await
            .expect("Expected successful connection to MongdoDB"),
    )
    .expect("Expected successful connection to MongdoDB");

    // Handle to connect to redis
    let redis_host_and_port = std::env::var("REDIS_URL").expect("Expected REDIS_URL in environment");
    let redis_username = std::env::var("REDIS_USER").expect("Expected REDIS_USER in environment");
    let redis_password = std::env::var("REDIS_PASSWORD").expect("Expected REDIS_PASSWORD in environment");
    let redis_url = format!(
        "redis://{}:{}@{}/0",
        redis_username,
        redis_password,
        redis_host_and_port
    );
    let redis_client = redis::Client::open(redis_url).expect("Expected to be able to connect to Redis");

    // All resources needed to handle a request not contained in the request's
    // URL, headers, or body.
    let state = AppState { 
        discord_public_key,
        discord_token,
        cookie_signing_key,
        _mongodb_client: mongodb_client,
        _redis_client: redis_client,
    };

    // Initialize the router for the app.
    let app = Router::new()
        .merge(SpaRouter::new("/assets", "assets"))
        .route("/discord", post(post_discord_handler))
        .route("/login", get(handle_login))
        .route("/login/callback", get(handle_login_callback))
        .with_state(state);

    // Start listening on port 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("unexpected error");
}
