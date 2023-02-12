/// The login callback route.
pub mod callback;


use axum::{response::{Redirect}, extract::State};
use axum_extra::extract::{SignedCookieJar, cookie::{Cookie}};
use rand::Rng;
use time::Duration;

use crate::AppState;

/// Handles GET requests made to daiklave.app/login by setting the user's cookie
/// and redirecting them to the Discord OAuth2 endpoint for validation.
pub async fn get_login(State(state): State<AppState>, jar: SignedCookieJar) -> (SignedCookieJar, Redirect) {
    // Generate a new random string to use as discord state
    let discord_state_nonce = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(20).map(char::from).collect::<String>();

    // Construct the URL to point the user to
    let base_discord_oauth = "https://discord.com/api/oauth2/authorize";
    let response_type = "response_type=code";
    let client_id = format!("client_id={}", state.discord_token);
    let scopes = "scope=identify";
    let discord_state = format!("state={}", &discord_state_nonce);
    let callback = "redirect_uri=https%3A%2F%2Fwpizmuff3s.us-west-2.awsapprunner.com%2Flogin%2Fcallback%2F";
    let prompt = "prompt=none";
    let discord_redirect_url = format!(
        "{}?{}&{}&{}&{}&{}&{}",
        base_discord_oauth,
        response_type,
        client_id,
        scopes,
        discord_state,
        callback,
        prompt
    );

    // Add a new pending auth cookie lasting one hour
    let jar = jar
        .remove(Cookie::named("daiklaveAuth"))
        .add(Cookie::build("daiklavePendingAuth", discord_state_nonce)
            .max_age(Duration::hours(1))
            .http_only(true)
            .secure(true)
            .domain("https%3A%2F%2Fwpizmuff3s.us-west-2.awsapprunner.com")
            .finish()
        );

    // Redirect the user to the constructed url and tell them to update their 
    // cookies
    (jar, Redirect::to(&discord_redirect_url))
}