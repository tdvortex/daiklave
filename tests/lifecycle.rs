use daiklave::{
    armor::destroy_armor, character::destroy_character, create_player, destroy_player,
    merits::destroy_merits, update_character, weapons::destroy_weapons, Character,
};
use postcard::from_bytes;
use sqlx::PgPool;

mod fixtures;

use fixtures::{create_initial_character, validate_initial_character};

use crate::fixtures::{
    modify_character,
    serde::{validate_initial_character_serde, validate_modified_character_serde},
    validate_modified_character, validate_player_serde,
};

#[sqlx::test]
fn lifecycle() {
    dotenvy::dotenv().unwrap();
    let url = dotenvy::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&url).await.unwrap();

    // Setup: clean database
    sqlx::query!("DELETE FROM players")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query!("DELETE FROM armor")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query!("DELETE FROM merits")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query!("DELETE FROM weapons")
        .execute(&pool)
        .await
        .unwrap();

    // User inputs a username, Client serializes it
    let player_name = "Test Player Name".to_owned();
    let send_bytes = postcard::to_allocvec(&player_name).unwrap();

    // Server deserializes it and creates a new player with that name
    let player = create_player(&pool, from_bytes::<String>(&send_bytes).unwrap().clone())
        .await
        .unwrap();
    assert_eq!(&player_name.as_str(), &player.name());

    // Server serializes player result and sends it back to the client
    validate_player_serde(&player);

    // Client (in isolation) creates a character and subcomponents
    let character = create_initial_character(&player);
    validate_initial_character(&player, &character, false);

    // Client builds, serializes, and sends to server
    // Server deserializes character
    validate_initial_character_serde(&player, &character, false);

    // Server inserts character and retrieves after updating
    let mut character: Character = update_character(&pool, &character).await.unwrap();
    validate_initial_character(&player, &character, true);

    // Server serializes and sends character to client
    // Client deserializes character and modifies it
    validate_initial_character_serde(&player, &character, true);
    modify_character(&mut character);
    validate_modified_character(&player, &character);

    // Client reserializes character and sends to server
    // Server deserializes, reconciles, inserts, and extracts
    validate_modified_character_serde(&player, &character);
    let character = update_character(&pool, &character).await.unwrap();

    // Client deserializes
    validate_modified_character(&player, &character);

    // Client sends delete character order
    // Server deletes character
    destroy_character(&pool, *character.id()).await.unwrap();

    // Character should not exist
    assert!(
        sqlx::query!("SELECT id FROM characters WHERE id = $1", *character.id())
            .fetch_optional(&pool)
            .await
            .unwrap()
            .is_none()
    );

    // Custom items should not exist
    assert!(sqlx::query!(
        "SELECT id FROM armor WHERE creator_id = $1",
        *character.id()
    )
    .fetch_optional(&pool)
    .await
    .unwrap()
    .is_none());

    assert!(sqlx::query!(
        "SELECT id FROM weapons WHERE creator_id = $1",
        *character.id()
    )
    .fetch_optional(&pool)
    .await
    .unwrap()
    .is_none());

    assert!(sqlx::query!(
        "SELECT id FROM merits WHERE creator_id = $1",
        *character.id()
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

    let unarmed_and_knife_ids =
        sqlx::query!("SELECT id FROM weapons WHERE name = 'Knife' OR name = 'Unarmed'")
            .fetch_all(&pool)
            .await
            .unwrap()
            .into_iter()
            .map(|record| record.id)
            .collect::<Vec<i32>>();
    assert!(unarmed_and_knife_ids.len() == 2);

    let merit_ids = sqlx::query!(
        "SELECT id FROM merits WHERE name = 'Martial Artist' OR name = 'Language' OR name = 'Danger Sense' OR name = 'Artifact'"
    ).fetch_all(&pool).await.unwrap().into_iter().map(|record| record.id).collect::<Vec<i32>>();

    assert!(merit_ids.len() == 4);

    // Client sends delete player order
    // Server deletes player
    destroy_player(&pool, player.id()).await.unwrap();

    // Player should not exist
    assert!(
        sqlx::query!("SELECT * FROM players WHERE id = $1", player.id())
            .fetch_optional(&pool)
            .await
            .unwrap()
            .is_none()
    );

    // Clean up database to end test
    destroy_armor(&pool, &[silken_armor_id]).await.unwrap();
    destroy_weapons(&pool, &unarmed_and_knife_ids)
        .await
        .unwrap();
    destroy_merits(&pool, &merit_ids).await.unwrap();

    // Confirm database is clean
    assert!(sqlx::query!(
        "SELECT id FROM armor WHERE name = 'Silken Armor' OR name = 'Stolen Guard''s Armor'"
    )
    .fetch_optional(&pool)
    .await
    .unwrap()
    .is_none());

    assert!(
        sqlx::query!("SELECT id FROM weapons WHERE name = 'Knife' OR name = 'Unarmed'")
            .fetch_optional(&pool)
            .await
            .unwrap()
            .is_none()
    );

    assert!(sqlx::query!(
        "SELECT id FROM merits WHERE name = 'Martial Artist' OR name = 'Language' OR name = 'Danger Sense' OR name = 'Artifact'"
    ).fetch_optional(&pool)
        .await
        .unwrap()
        .is_none());
}
