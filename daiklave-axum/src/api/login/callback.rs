use axum::http::StatusCode;
use axum::{
    extract::{Query, State},
    response::Redirect,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::SignedCookieJar;
use serde::Deserialize;
use serde_json::Value;
use serenity::all::User;
use time::Duration;

use crate::AppState;

/// The expected format of a Discord access code grant querystring.
#[derive(Deserialize)]
pub struct DiscordCodeGrant {
    code: String,
    state: String,
}

const DISCORD_API_URL_BASE: &str = "https://discord.com/api/v10/";

/// Handles GET requests received after the user returns from Discord's OAuth2
/// endpoint with an authorization code.
pub async fn get_login_callback(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    code_grant: Query<DiscordCodeGrant>,
) -> Result<(SignedCookieJar, Redirect), StatusCode> {
    // Verify that the user has a valid daiklavePendingAuth cookie
    let pending_auth = jar
        .get("daiklavePendingAuth")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify that the value in the daiklavePendingAuth cookie matches Discord's reported state
    let expected_state = pending_auth.value();
    if expected_state != &code_grant.state {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Exchange the auth code for an access token
    let discord_token_url = format!("{}{}", DISCORD_API_URL_BASE, "/oauth2/token");
    let form = [
        ("client_id", state.discord_token.as_str()),
        ("client_secret", &state.discord_client_secret.as_str()),
        ("grant_type", "authorization_code"),
        ("code", code_grant.code.as_str()),
        (
            "redirect_uri",
            "https%3A%2F%2Fwpizmuff3s.us-west-2.awsapprunner.com%2Flogin%2Fcallback%2F",
        ),
    ];

    let value = state
        .reqwest_client
        .post(discord_token_url)
        .form(&form)
        .send()
        .await
        // Something went wrong with the outgoing request
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?
        .json::<Value>()
        .await
        // Something went wrong trying to deserialize Discord's response
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let access_token = value
        .get("access_token")
        // Discord didn't give us an access token, the code must be bad
        .ok_or(StatusCode::UNAUTHORIZED)?
        .as_str()
        // "access_token" exists but the value isn't a string
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Use the access token to get the user's Discord snowflake
    let discord_me_url = format!("{}{}", DISCORD_API_URL_BASE, "users/@me");

    let user_id = state
        .reqwest_client
        .get(discord_me_url)
        .bearer_auth(access_token)
        .send()
        .await
        // Something went wrong with the outgoing request
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?
        .json::<User>()
        .await
        // Something went wrong trying to deserialize the User response
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?
        .id;

    // Authentication successful!
    // Clear the pending auth cookie and add a 1-year login cookie
    let jar = jar.remove(Cookie::named("daiklavePendingAuth")).add(
        Cookie::build("daiklaveAuth", hex::encode(user_id.0.get().to_be_bytes()))
            .domain("https%3A%2F%2Fwpizmuff3s.us-west-2.awsapprunner.com")
            .max_age(Duration::days(365))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .finish(),
    );

    // Redirect the user back to the homepage
    Ok((jar, Redirect::to("/")))
}
