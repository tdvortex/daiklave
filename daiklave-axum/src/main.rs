#![warn(missing_docs)]
//! daiklave-axum is the binary for the HTTP server component of the Daiklave
//! app. It is responsible for handing Discord webhook interactions; serving
//! static content for the Yew application; and serving API requests from the
//! Yew client (and potentially other 3rd party Exalted tools.)


/// The module responsible for parsing incoming requests to the API, and 
/// providing a suitable HTTP response and status code to the client 
/// (typically the daiklave-yew frontend).
pub mod api;

mod build_state;
use build_state::build_state;

/// The module responsible for parsing incoming POST requests from Discord for
/// interactions, and providing a 200 OK response with a message payload to 
/// communicate the result of the interaction to the user.
pub mod discord;

/// The module which processes the shared application logic of the API and
/// Discord interaction interfaces. The functions here do not return a complete
/// response, but simply a Result type that conveys whether the action was
/// successful and if not, why not.
pub mod shared;


use std::net::SocketAddr;

use axum::{routing::{post, get}, Router, extract::FromRef};
use axum_extra::routing::SpaRouter;

use crate::{discord::post_discord, api::{get_login, get_login_callback}};
/// Any handles or resources not tied to an individual request.
#[derive(Clone)]
pub struct AppState {
    /// Public key for verifying incoming POST requests from Discord
    pub discord_public_key: ed25519_dalek::PublicKey,
    /// Discord Id for this application
    pub discord_token: String,
    /// Discord secret for token exchanges
    pub discord_client_secret: String,
    /// Key for signing authentication cookies
    pub cookie_signing_key: axum_extra::extract::cookie::Key,
    /// Client for outgoing network requests
    pub reqwest_client: reqwest::Client,
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

    // All resources needed to handle a request not contained in the request's
    // URL, headers, or body.
    let state = build_state().await;

    // Initialize the router for the app.
    let app = Router::new()
        .merge(SpaRouter::new("/assets", "assets"))
        .route("/discord", post(post_discord))
        .route("/login", get(get_login))
        .route("/login/callback", get(get_login_callback))
        .with_state(state);

    // Start listening on port 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("unexpected error");
}
