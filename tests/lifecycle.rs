use postcard::from_bytes;
use sqlx::postgres::PgPool;
use exalted_3e_gui::{create_player, destroy_player, player::Player, Character, character::{Willpower, ExperiencePoints}, update_character};

#[sqlx::test]
fn lifecycle() {
    dotenvy::dotenv().unwrap();
    let url = dotenvy::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&url).await.unwrap();

    // User inputs a username, Client serializes it
    let player_name = "Test Player Name".to_owned();
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
    let initial_character = Character::create()
        .with_player(receive_player.clone())
        .with_name("Test Character Name".to_owned())
        .with_concept("A character for testing purposes".to_owned())
        .with_willpower(Willpower {
            current: 5,
            maximum: 6,
        })
        .with_experience(ExperiencePoints {
            current: 15,
            total: 15,
        })
        .build()
    .unwrap();
    assert!(initial_character.id().is_none());
    assert_eq!(initial_character.player(), &receive_player);
    assert_eq!(&initial_character.name, "Test Character Name");
    assert_eq!(initial_character.concept.as_deref(), Some("A character for testing purposes"));
    assert_eq!(initial_character.willpower, 
        Willpower {
        current: 5,
        maximum: 6,
    });
    assert_eq!(initial_character.experience, 
        ExperiencePoints {
            current: 15,
            total: 15,
    });
    assert_eq!(initial_character.experience, initial_character.experience);    

    // Client builds, serializes, and sends to server
    let send_bytes = postcard::to_allocvec(&initial_character).unwrap();

    // Server deserializes, inserts, then extracts the character
    let receive_character: Character = from_bytes(&send_bytes).unwrap();
    assert!(receive_character.id().is_none());
    assert_eq!(receive_character.player(), &receive_player);
    assert_eq!(receive_character.name, initial_character.name);
    assert_eq!(receive_character.concept, initial_character.concept);
    assert_eq!(receive_character.willpower, initial_character.willpower);
    assert_eq!(receive_character.experience, initial_character.experience);

    let post_insert_character: Character = update_character(&pool, &receive_character).await.unwrap();
    assert!(post_insert_character.id().is_some());
    assert_eq!(receive_character.player(), post_insert_character.player());
    assert_eq!(receive_character.name, post_insert_character.name);
    assert_eq!(receive_character.concept, post_insert_character.concept);
    assert_eq!(receive_character.willpower, post_insert_character.willpower);
    assert_eq!(receive_character.experience, post_insert_character.experience);

    // Server serializes and sends character to client
    let send_bytes = postcard::to_allocvec(&post_insert_character).unwrap();

    // Client deserializes character and modifies it
    let fetched_character: Character = from_bytes(&send_bytes).unwrap();
    assert_eq!(fetched_character.id(), post_insert_character.id());
    assert_eq!(fetched_character.player(), post_insert_character.player());
    assert_eq!(fetched_character.name, post_insert_character.name);
    assert_eq!(fetched_character.concept, post_insert_character.concept);
    assert_eq!(fetched_character.willpower, post_insert_character.willpower);
    assert_eq!(fetched_character.experience, post_insert_character.experience);

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

    // Character should not exist
    assert!(sqlx::query!(
        "SELECT id FROM characters WHERE id = $1", fetched_character.id().unwrap()
    ).fetch_optional(&pool).await.unwrap().is_none());

    // End state: non-custom elements remain in database
    // Clean up database to end test
}