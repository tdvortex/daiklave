mod abilities;
mod armor;
mod attributes;
mod character;
mod health;
mod intimacies;
mod weapons;
pub use armor::{create_initial_armor, validate_initial_armor_items};
pub use abilities::validate_initial_abilities;
pub use character::create_initial_base_character;
pub use health::create_initial_health;

use std::collections::{HashSet};

use exalted_3e_gui::{
    armor::{destroy_armor},
    create_player,
    destroy_player,
    intimacies::{Intimacy, IntimacyLevel, IntimacyType},
    player::Player,
    update_character,
    weapons::{destroy_weapons},
    Character,
};
use postcard::from_bytes;
use sqlx::postgres::PgPool;

use crate::fixtures::{character::validate_initial_base_character, weapons::validate_initial_weapons};

use self::{attributes::{create_initial_attributes, validate_initial_attributes}, abilities::create_intitial_abilities, intimacies::{create_initial_intimacies, validate_initial_intimacies}, weapons::create_initial_weapons, health::validate_initial_health};

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

pub fn validate_initial_character(player: &Player, initial_character: &Character, should_have_id: bool) {
    validate_initial_base_character(player, initial_character, should_have_id);
    validate_initial_attributes(&initial_character.attributes);
    validate_initial_abilities(&initial_character.abilities);
    validate_initial_intimacies(&initial_character.intimacies, should_have_id);
    validate_initial_health(&initial_character.health);
    validate_initial_armor_items(&initial_character.armor, should_have_id);
    validate_initial_weapons(&initial_character.weapons, should_have_id);
}


fn check_intimacies_except_id(left: &Vec<Intimacy>, right: &Vec<Intimacy>) {
    assert!(
        left.iter()
            .map(|i| (i.intimacy_level, i.intimacy_type, i.description.as_str()))
            .collect::<HashSet<_>>()
            == right
                .iter()
                .map(|i| (i.intimacy_level, i.intimacy_type, i.description.as_str()))
                .collect::<HashSet<_>>()
    )
}





fn validate_deserialization(preserialized: &Character, postserialized: &Character) {
    assert_eq!(preserialized.id(), postserialized.id());
    assert_eq!(preserialized.player(), postserialized.player());
    assert_eq!(preserialized.name, postserialized.name);
    assert_eq!(preserialized.concept, postserialized.concept);
    assert_eq!(preserialized.willpower, postserialized.willpower);
    assert_eq!(preserialized.experience, postserialized.experience);
    assert_eq!(preserialized.health, postserialized.health);
    check_intimacies_except_id(&preserialized.intimacies, &postserialized.intimacies);
}


pub async fn lifecycle() {
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
    let initial_character = create_initial_character(&receive_player);
    validate_initial_character(&player, &initial_character, false);

    assert_eq!(
        initial_character
            .intimacies
            .iter()
            .collect::<HashSet<&Intimacy>>(),
        [
            Intimacy::new(
                IntimacyLevel::Defining,
                IntimacyType::Principle,
                "Never stand idle against injustice".to_owned(),
                None
            ),
            Intimacy::new(
                IntimacyLevel::Major,
                IntimacyType::Tie,
                "Ragara Tirnis (Love)".to_owned(),
                None
            ),
            Intimacy::new(
                IntimacyLevel::Major,
                IntimacyType::Tie,
                "Mask of Winters (Revenge)".to_owned(),
                None
            ),
            Intimacy::new(
                IntimacyLevel::Minor,
                IntimacyType::Tie,
                "Street Vendors (Camaraderie)".to_owned(),
                None
            )
        ]
        .iter()
        .collect()
    );
    assert!(initial_character
        .intimacies
        .iter()
        .all(|i| i.id().is_none()));

    validate_initial_armor_items(&initial_character.armor, false);
    validate_initial_weapons(&initial_character.weapons, false);

    // Client builds, serializes, and sends to server
    let send_bytes = postcard::to_allocvec(&initial_character).unwrap();

    // Server deserializes character
    let receive_character: Character = from_bytes(&send_bytes).unwrap();
    validate_deserialization(&initial_character, &receive_character);
    validate_initial_abilities(&receive_character.abilities);
    assert!(receive_character
        .intimacies
        .iter()
        .all(|i| i.id().is_none()));
    validate_initial_armor_items(&receive_character.armor, false);
    validate_initial_weapons(&receive_character.weapons, false);

    // Server inserts character and retrieves after updating
    let post_insert_character: Character =
        update_character(&pool, &receive_character).await.unwrap();
    assert!(post_insert_character.id().is_some());
    assert_eq!(receive_character.player(), post_insert_character.player());
    assert_eq!(receive_character.name, post_insert_character.name);
    assert_eq!(receive_character.concept, post_insert_character.concept);
    assert_eq!(receive_character.willpower, post_insert_character.willpower);
    assert_eq!(
        receive_character.experience,
        post_insert_character.experience
    );
    assert_eq!(
        receive_character.attributes,
        post_insert_character.attributes
    );
    validate_initial_abilities(&post_insert_character.abilities);
    assert!(post_insert_character
        .intimacies
        .iter()
        .all(|i| i.id().is_some()));
    assert_eq!(&receive_character.health, &post_insert_character.health);
    validate_initial_armor_items(&post_insert_character.armor, true);
    validate_initial_weapons(&post_insert_character.weapons, true);

    // Server serializes and sends character to client
    let send_bytes = postcard::to_allocvec(&post_insert_character).unwrap();

    // Client deserializes character and modifies it
    let fetched_character: Character = from_bytes(&send_bytes).unwrap();
    validate_deserialization(&initial_character, &receive_character);
    validate_initial_abilities(&fetched_character.abilities);
    assert_eq!(
        fetched_character
            .intimacies
            .iter()
            .map(|i| i.id().unwrap())
            .collect::<HashSet<i32>>(),
        post_insert_character
            .intimacies
            .iter()
            .map(|i| i.id().unwrap())
            .collect::<HashSet<i32>>(),
    );
    validate_initial_armor_items(&fetched_character.armor, true);
    validate_initial_weapons(&fetched_character.weapons, true);

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
        fetched_character.id().unwrap()
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
        fetched_character.id().unwrap()
    )
    .fetch_optional(&pool)
    .await
    .unwrap()
    .is_none());

    assert!(sqlx::query!(
        "SELECT id FROM weapons WHERE creator_id = $1",
        fetched_character.id().unwrap()
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
