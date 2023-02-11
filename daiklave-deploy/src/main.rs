mod campaign;
mod version;

use std::collections::HashMap;

use serde_json::Value;

const DISCORD_API_URL_BASE: &str = "https://discord.com/api/v10/";

fn main() {
    dotenvy::dotenv().unwrap();

    // Get an access token so that we can update commands
    let http_client = reqwest::blocking::Client::new();

    let client_secret = std::env::var("DISCORD_CLIENT_SECRET")
        .expect("Expected DISCORD_CLIENT_SECRET in environment");
    let application_id =
        std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in environment");
    let token_url = format!("{}{}", DISCORD_API_URL_BASE, "/oauth2/token");
    let mut payload = HashMap::new();
    payload.insert("grant_type", "client_credentials");
    payload.insert("scope", "applications.commands.update");

    let request = http_client
        .post(&token_url)
        .form(&payload)
        .basic_auth(&application_id, Some(&client_secret))
        .build()
        .unwrap();

    println!("{:?}", request.headers());
    println!(
        "{}",
        std::str::from_utf8(request.body().unwrap().as_bytes().unwrap()).unwrap()
    );

    let json_response = http_client
        .execute(request)
        .expect("Expected client credentials request to succeed")
        .json::<Value>()
        .expect("Expected client credentials response to be json");

    println!("{}", json_response);

    let access_token = json_response
        .get("access_token")
        .expect("Expected client credentials response to include an access token")
        .as_str()
        .expect("Expected client credentials response to include an access token");

    println!("{:?}", access_token);

    // Use that token to set application commands
    // let global_application_commands_url = format!(
    //     "{}{}{}{}",
    //     DISCORD_API_URL_BASE,
    //     "applications/",
    //     application_id,
    //     "/commands"
    // );
    // println!("{}", &global_application_commands_url);

    let dev_guild_id =
        std::env::var("DISCORD_TEST_GUILD").expect("Expected DISCORD_TEST_GUILD in environment");

    let guild_application_commands_url = format!(
        "{}{}{}{}{}{}",
        DISCORD_API_URL_BASE,
        "applications/",
        application_id,
        "/guilds/",
        dev_guild_id,
        "/commands"
    );
    println!("{}", &guild_application_commands_url);

    let guild_commands = vec![version::version(), campaign::campaign()];
    let request = http_client
        .put(&guild_application_commands_url)
        .bearer_auth(access_token)
        .json(&guild_commands)
        .build()
        .unwrap();

    println!("{:?}", request.headers());
    println!(
        "{}",
        std::str::from_utf8(request.body().unwrap().as_bytes().unwrap()).unwrap()
    );

    let response = http_client.execute(request).unwrap();

    println!("{}", response.status());
    println!("{}", response.text().unwrap());
}
