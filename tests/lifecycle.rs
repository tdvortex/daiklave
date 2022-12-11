use exalted_3e_gui::{
    armor::destroy_armor, create_player, destroy_player, player::Player, update_character,
    weapons::destroy_weapons, Character,
};
use postcard::from_bytes;
use sqlx::PgPool;

mod fixtures;

use fixtures::{create_initial_character, validate_initial_character};

use crate::fixtures::{serde::validate_initial_character_serde, validate_player_serde};

#[sqlx::test]
fn lifecycle() {
    dotenvy::dotenv().unwrap();
    let url = dotenvy::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&url).await.unwrap();

    // User inputs a username, Client serializes it
    let player_name = "Test Player Name".to_owned();
    let send_bytes = postcard::to_allocvec(&player_name).unwrap();

    // Server deserializes it and creates a new player with that name
    let player = create_player(&pool, from_bytes::<String>(&send_bytes).unwrap().clone())
        .await
        .unwrap();
    assert_eq!(&player_name.as_str(), &player.name());

    // Server serializes player result and sends it back to the client
    let player: Player = validate_player_serde(&player);

    // Client (in isolation) creates a character and subcomponents
    let character = create_initial_character(&player);
    validate_initial_character(&player, &character, false);

    // Client builds, serializes, and sends to server
    // Server deserializes character
    let character = validate_initial_character_serde(&player, &character, false);

    // Server inserts character and retrieves after updating
    let character: Character = update_character(&pool, &character).await.unwrap();
    validate_initial_character(&player, &character, true);

    // Server serializes and sends character to client
    // Client deserializes character and modifies it
    let character = validate_initial_character_serde(&player, &character, true);

    // Client runs all getters on the character
    // Client runs all setters on the character
    // Client reserializes character and sends to server
    // Server deserializes, reconciles, inserts, extracts, and reserializes
    // Client deserializes
    // Client sends delete player order
    // Server deletes player, sends confirmation
    destroy_player(&pool, player.id()).await.unwrap();

    // Confirm end state
    // Player should not exist
    assert!(
        sqlx::query!("SELECT * FROM players WHERE id = $1", player.id())
            .fetch_optional(&pool)
            .await
            .unwrap()
            .is_none()
    );

    // Character should not exist
    assert!(sqlx::query!(
        "SELECT id FROM characters WHERE id = $1",
        character.id().unwrap()
    )
    .fetch_optional(&pool)
    .await
    .unwrap()
    .is_none());

    // Book referenced items should still exist
    let silken_armor_id = sqlx::query!("SELECT id FROM armor WHERE name = 'Silken Armor'")
        .fetch_optional(&pool)
        .await
        .unwrap()
        .unwrap()
        .id;

    let knife_id = sqlx::query!("SELECT id FROM weapons WHERE name = 'Knife'")
        .fetch_optional(&pool)
        .await
        .unwrap()
        .unwrap()
        .id;

    // Custom items should not
    assert!(sqlx::query!(
        "SELECT id FROM armor WHERE creator_id = $1",
        character.id().unwrap()
    )
    .fetch_optional(&pool)
    .await
    .unwrap()
    .is_none());

    assert!(sqlx::query!(
        "SELECT id FROM weapons WHERE creator_id = $1",
        character.id().unwrap()
    )
    .fetch_optional(&pool)
    .await
    .unwrap()
    .is_none());

    // Clean up database to end test
    destroy_armor(&pool, &[silken_armor_id]).await.unwrap();
    destroy_weapons(&pool, &[knife_id]).await.unwrap();

    // Confirm database is clean
    assert!(
        sqlx::query!("SELECT id FROM armor WHERE name = 'Silken Armor'")
            .fetch_optional(&pool)
            .await
            .unwrap()
            .is_none()
    );

    assert!(sqlx::query!("SELECT id FROM weapons WHERE name = 'Knife'")
        .fetch_optional(&pool)
        .await
        .unwrap()
        .is_none());
}
