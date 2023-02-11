use hex::decode;

use crate::AppState;

pub async fn build_state() -> AppState {
    // Public key for verifying incoming POST requests from Discord
    let hex_string =
        std::env::var("DISCORD_PUBLIC_KEY").expect("Expected DISCORD_PUBLIC_KEY in environment");
    let hex_bytes =
        decode(hex_string).expect("Expected DISCORD_PUBLIC_KEY to be valid hexadecimal");

    let discord_public_key = ed25519_dalek::PublicKey::from_bytes(&hex_bytes)
        .expect("Expected DISCORD_PUBLIC_KEY to be a valid ed25519 public key");

    // Discord Id for this application
    let discord_token = std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in environment");

    // Discord secret for token exchanges
    let discord_client_secret = std::env::var("DISCORD_CLIENT_SECRET")
        .expect("Expected DISCORD_CLIENT_SECRET in environment");

    // Key for signing authentication cookies
    let hex_string = std::env::var("COOKIE_SIGNING_KEY").expect("Expected COOKIE_SIGNING_KEY in environment");
    let hex_bytes = decode(hex_string).expect("Expected COOKIE_SIGNING_KEY to be valid hexadecimal");
    let cookie_signing_key = axum_extra::extract::cookie::Key::from(&hex_bytes);

    // Client for outgoing network requests
    let reqwest_client = reqwest::Client::new();

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

    AppState { 
        discord_public_key,
        discord_token,
        discord_client_secret,
        cookie_signing_key,
        reqwest_client,
        _mongodb_client: mongodb_client,
        _redis_client: redis_client,
    }
}