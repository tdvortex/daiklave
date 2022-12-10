use postcard::from_bytes;
use sqlx::postgres::PgPool;
use exalted_3e_gui::create_player;

#[sqlx::test]
fn lifecycle() {
    dotenvy::dotenv().unwrap();
    let url = dotenvy::var("DATABASE_URL").unwrap();
    let _pool = PgPool::connect(&url).await.unwrap();

    // User inputs a username, Client serializes it
    let player_name = "Ralph Waldo Emerson".to_owned();
    let send_bytes = postcard::to_allocvec(&player_name).unwrap();
    
    // Server deserializes it and creates a new player
    let receive_name: String = from_bytes(&send_bytes).unwrap();
    assert_eq!(receive_name, player_name);


    // Server serializes player result and sends it back to the client
    // Client deserializes and extracts player ID
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
    // End state: non-custom elements remain in database
    // Clean up database to end test
}