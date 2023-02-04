use serde_json::{json, Value};
use serenity::builder::CreateCommand;

use super::DISCORD_API_URL_BASE;

/// Creates all app commands, both globally and in the dev server.
pub async fn create_app_commands() {
    // Register all application commands in the dev server.
    // Get an access token so that we can update commands
    let http_client = reqwest::Client::new();

    let client_secret = std::env::var("DISCORD_CLIENT_SECRET").expect("Expected DISCORD_CLIENT_SECRET in environment");
    let application_id = std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in environment");
    let token_url = format!("{}{}", DISCORD_API_URL_BASE, "/oauth2/token");
    let payload = json!({
        "grant_type": "client_credentials",
        "scope": "identify applications.commands.update",
    });

    let access_token = http_client
        .post(&token_url)
        .json(&payload)
        .basic_auth(&client_secret, Some(&application_id))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await
        .expect("Expected client credentials request to succeed")
        .json::<Value>()
        .await
        .expect("Expected client credentials response to be json")
        .get("access_token")
        .expect("Expected client credentials response to include an access token")
        .to_string();

    // Use that token to set application commands
    let dev_guild_id = std::env::var("DISCORD_TEST_GUILD").expect("Expected DISCORD_TEST_GUILD in environment");

    let guild_application_commands_url = format!(
        "{}{}{}{}{}{}",
        DISCORD_API_URL_BASE,
        "/applications/",
        application_id,
        "/guilds/",
        dev_guild_id,
        "/commands"
    );

    let commands = vec![
        CreateCommand::new("version").description("Returns the Daiklave version being used")
    ];

    http_client.put(&guild_application_commands_url).bearer_auth(access_token).json(&commands).send().await.expect("Expected to be able to update dev guild app commands");
}