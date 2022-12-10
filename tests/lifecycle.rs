use postcard::from_bytes;
use sqlx::postgres::PgPool;
use exalted_3e_gui::{create_player, destroy_player, player::Player};

#[sqlx::test]
fn lifecycle() {
    dotenvy::dotenv().unwrap();
    let url = dotenvy::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&url).await.unwrap();

    // User inputs a username, Client serializes it
    let player_name = "Ralph Waldo Emerson".to_owned();
    let send_bytes = postcard::to_allocvec(&player_name).unwrap();
    
    // Server deserializes it and creates a new player
    let receive_name: String = from_bytes(&send_bytes).unwrap();
    assert_eq!(receive_name, player_name);

    let player = create_player(&pool, receive_name.clone()).await.unwrap();
    assert_eq!(&receive_name.as_str(), &player.name());

    // Server serializes player result and sends it back to the client
    let send_bytes = postcard::to_allocvec(&player).unwrap();
    // Client deserializes and extracts player ID
    let receive_player: Player = from_bytes(&send_bytes).unwrap();
    assert_eq!(player_name.as_str(), receive_player.name());
    assert_eq!(player.id(), receive_player.id());

    // Client (in isolation) creates a character and subcomponents
    // Client builds, serializes, and sends to server
    // Server deserializes, inserts, then extracts the character
    // Server serializes and sends character to client
    // Client deserializes character and modifies it
    // Client reserializes character and sends to server
    // Server deserializes, reconciles, inserts, extracts, and reserializes
    // Client deserializes
    // Client sends delete player order
    // Server deletes player, sends confirmation
    destroy_player(&pool, player.id()).await.unwrap();

    // Confirm end state
    // Player should not exist
    assert!(sqlx::query!(
        "SELECT * FROM players WHERE id = $1", player.id()
    ).fetch_optional(&pool).await.unwrap().is_none());

    // End state: non-custom elements remain in database
    // Clean up database to end test
}