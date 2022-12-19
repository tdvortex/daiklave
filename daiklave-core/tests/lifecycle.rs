mod fixtures;

use daiklave_core::{player::Player, id::Id};
use fixtures::{create_initial_character, validate_initial_character};

use crate::fixtures::{
    modify_character,
    serde::{validate_initial_character_serde, validate_modified_character_serde},
    validate_modified_character, validate_player_serde,
};

#[test]
fn lifecycle() {
    // Setup: clean database
    // User inputs a username, Client serializes it
    let player_name = "Test Player Name".to_owned();

    // Server deserializes it and creates a new player with that name
    let player = Player::new(Id::Database(123456789), player_name.clone());
    assert_eq!(&player_name.as_str(), &player.name());

    // Server serializes player result and sends it back to the client
    validate_player_serde(&player);

    // Client (in isolation) creates a character and subcomponents
    let mut character = create_initial_character(&player);
    validate_initial_character(&player, &character, false);

    // Client builds, serializes, and sends to server
    // Server deserializes character
    // Server inserts character and retrieves after updating
    // Server serializes and sends character to client
    // Client deserializes character and modifies it
    validate_initial_character_serde(&player, &character, false);
    modify_character(&mut character);
    validate_modified_character(&player, &character);

    // Client reserializes character and sends to server
    // Server deserializes, reconciles, inserts, and extracts
    validate_modified_character_serde(&player, &character);

    // Client sends delete character order
    // Server deletes character

    // Character should not exist
    // Custom items should not exist
    // Book referenced items should still exist
    // Client sends delete player order
    // Server deletes player
    // Player should not exist
    // Clean up database to end test
    // Confirm database is clean
}
