mod abilities;
mod armor;
mod attributes;
mod character;
mod health;
mod intimacies;
mod serde;
mod weapons;
use exalted_3e_gui::{
    armor::destroy_armor, create_player, destroy_player, player::Player,
    update_character, weapons::destroy_weapons, Character,
};
use postcard::from_bytes;
use sqlx::postgres::PgPool;

use crate::fixtures::{
    character::validate_initial_base_character,
    serde::{validate_initial_character_serde, validate_player_serde},
    weapons::validate_initial_weapons,
};

use self::{
    abilities::{create_intitial_abilities, validate_initial_abilities},
    attributes::{create_initial_attributes, validate_initial_attributes},
    health::{validate_initial_health, create_initial_health},
    intimacies::{create_initial_intimacies, validate_initial_intimacies},
    weapons::create_initial_weapons, character::create_initial_base_character, armor::{create_initial_armor, validate_initial_armor_items},
};

pub fn create_initial_character(player: &Player) -> Character {
    let mut builder = create_initial_base_character(player);
    builder = create_initial_attributes(builder);
    builder = create_intitial_abilities(builder);
    builder = create_initial_intimacies(builder);
    builder = create_initial_health(builder);
    builder = create_initial_armor(builder);
    builder = create_initial_weapons(builder);

    builder.build().unwrap()
}

pub fn validate_initial_character(
    player: &Player,
    initial_character: &Character,
    should_have_id: bool,
) {
    validate_initial_base_character(player, initial_character, should_have_id);
    validate_initial_attributes(&initial_character.attributes);
    validate_initial_abilities(&initial_character.abilities);
    validate_initial_intimacies(&initial_character.intimacies, should_have_id);
    validate_initial_health(&initial_character.health);
    validate_initial_armor_items(&initial_character.armor, should_have_id);
    validate_initial_weapons(&initial_character.weapons, should_have_id);
}

pub async fn lifecycle() {
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
