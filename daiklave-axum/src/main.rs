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

/// A module representing the format of documents in MongdoDb collections.
pub mod mongo;

/// The module which processes the shared application logic of the API and
/// Discord interaction interfaces. The functions here do not return a complete
/// response, but simply a Result type that conveys whether the action was
/// successful and if not, why not.
pub mod shared;

use std::net::SocketAddr;

use axum::{
    extract::FromRef,
    routing::{delete, get, patch, post, put},
    Router,
};
use axum_extra::routing::SpaRouter;

use crate::{
    api::{
        campaigns::{
            campaign::{get_campaign, players::player::delete_campaign_player},
            get_campaigns,
        },
        characters::{
            character::{delete_character, get_character, patch_character, put_character},
            post_character,
        },
        login::{callback::get_login_callback, get_login},
    },
    discord::post_discord,
};
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
    pub mongodb_client: mongodb::Client,
    /// The name of the MongoDb database to use (like "dev" or "prod")
    pub mongodb_database_name: String,
    /// Multiplexed, async, managed Redis connection pool
    pub redis_connection_manager: redis::aio::ConnectionManager,
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
        .route("/campaigns", get(get_campaigns))
        .route("/campaigns/:campaign", get(get_campaign))
        .route(
            "/campaigns/:campaign/players/:player",
            delete(delete_campaign_player),
        )
        .route("/campaigns/:campaign/characters", post(post_character))
        .route(
            "/campaigns/:campaign/characters/:character",
            delete(delete_character),
        )
        .route(
            "/campaigns/:campaign/characters/:character",
            get(get_character),
        )
        .route(
            "/campaigns/:campaign/characters/:character",
            patch(patch_character),
        )
        .route(
            "/campaigns/:campaign/characters/:character",
            put(put_character),
        )
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
