mod abilities;
mod attributes;
mod character;
pub use character::create_initial_base_character;

mod initial_character_definition;
pub use initial_character_definition::create_initial_character;
use std::collections::{HashMap, HashSet};

use exalted_3e_gui::{
    abilities::{Abilities, AbilityNameNoSubskill},
    armor::{destroy_armor, Armor, ArmorTag},
    attributes::AttributeName,
    create_player,
    data_source::{BookReference, DataSource},
    destroy_player,
    health::{DamageLevel, WoundPenalty},
    intimacies::{Intimacy, IntimacyLevel, IntimacyType},
    player::Player,
    update_character,
    weapons::{destroy_weapons, EquipHand, RangeBand, WeaponTag, Weapons},
    Character,
};
use postcard::from_bytes;
use sqlx::postgres::PgPool;

use crate::fixtures::character::validate_initial_base_character;

fn check_initial_abilities(abilities: &Abilities) {
    vec![
        (AbilityNameNoSubskill::Archery, None, 0, None),
        (AbilityNameNoSubskill::Athletics, None, 2, None),
        (AbilityNameNoSubskill::Awareness, None, 4, None),
        (AbilityNameNoSubskill::Brawl, None, 1, None),
        (AbilityNameNoSubskill::Bureaucracy, None, 0, None),
        (
            AbilityNameNoSubskill::Craft,
            Some("Weapon Forging"),
            1,
            Some(&(["Sharpening Blades".to_owned()].into())),
        ),
        (AbilityNameNoSubskill::Dodge, None, 3, None),
        (AbilityNameNoSubskill::Integrity, None, 2, None),
        (AbilityNameNoSubskill::Investigation, None, 0, None),
        (AbilityNameNoSubskill::Larceny, None, 0, None),
        (AbilityNameNoSubskill::Linguistics, None, 1, None),
        (AbilityNameNoSubskill::Lore, None, 0, None),
        (
            AbilityNameNoSubskill::MartialArts,
            Some("Single Point Shining Into Void Style"),
            4,
            Some(&(["Join Battle".to_owned()].into())),
        ),
        (AbilityNameNoSubskill::Medicine, None, 0, None),
        (AbilityNameNoSubskill::Melee, None, 0, None),
        (AbilityNameNoSubskill::Occult, None, 0, None),
        (AbilityNameNoSubskill::Performance, None, 0, None),
        (AbilityNameNoSubskill::Presence, None, 2, None),
        (AbilityNameNoSubskill::Resistance, None, 3, None),
        (AbilityNameNoSubskill::Ride, None, 0, None),
        (AbilityNameNoSubskill::Sail, None, 0, None),
        (
            AbilityNameNoSubskill::Socialize,
            None,
            2,
            Some(&(["Tavern Gossip".to_owned()].into())),
        ),
        (AbilityNameNoSubskill::Stealth, None, 0, None),
        (AbilityNameNoSubskill::Survival, None, 0, None),
        (AbilityNameNoSubskill::Thrown, None, 0, None),
        (
            AbilityNameNoSubskill::War,
            None,
            3,
            Some(&(["While Outnumbered".to_owned()].into())),
        ),
    ]
    .into_iter()
    .for_each(
        |(ability_name_no_subskill, subskill, expect_dots, expect_specialties)| {
            assert_eq!(
                abilities
                    .get(ability_name_no_subskill, subskill)
                    .unwrap()
                    .dots(),
                expect_dots
            );
            assert_eq!(
                abilities
                    .get(ability_name_no_subskill, subskill)
                    .unwrap()
                    .specialties(),
                expect_specialties
            );
        },
    );

    vec![
        (AbilityNameNoSubskill::Craft, Some("Does Not Exist")),
        (AbilityNameNoSubskill::MartialArts, Some("Does Not Exist")),
    ]
    .into_iter()
    .for_each(|(ability_name_no_subskill, subskill)| {
        assert!(abilities.get(ability_name_no_subskill, subskill).is_none());
    });
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

fn check_initial_armor_items(armor: &Armor, should_have_id: bool) {
    for (key, worn, item) in armor.iter() {
        match item.name() {
            "Straw Hat" => {
                assert!(worn);
                assert!(armor.get(key).unwrap().id().is_some() == should_have_id);
                assert_eq!(armor.get(key).unwrap().tags(), [ArmorTag::Light].into());
                if should_have_id {
                    assert!(match armor.get(key).unwrap().data_source() {
                        DataSource::Book(_) => panic!("should be custom"),
                        DataSource::Custom(None) => panic!("should have custom creator id"),
                        DataSource::Custom(Some(_)) => true,
                    });
                } else {
                    assert_eq!(
                        armor.get(key).unwrap().data_source(),
                        &DataSource::Custom(None)
                    );
                }
            }
            "Silken Armor" => {
                assert!(!worn);
                assert!(armor.get(key).unwrap().id().is_some() == should_have_id);
                assert_eq!(
                    armor.get(key).unwrap().tags(),
                    [
                        ArmorTag::Light,
                        ArmorTag::Artifact,
                        ArmorTag::Silent,
                        ArmorTag::Special
                    ]
                    .into()
                );
                assert_eq!(
                    armor.get(key).unwrap().data_source(),
                    &DataSource::Book(BookReference {
                        book_title: "Core Rulebook".to_owned(),
                        page_number: 600
                    })
                );
            }
            wrong => panic!("Unknown armor name: {}", wrong),
        }
    }
}

fn check_initial_weapons(weapons: &Weapons, should_have_id: bool) {
    for (key, maybe_hand, weapon_ref) in weapons.iter() {
        match weapon_ref.name() {
            "Knife" => {
                assert!(maybe_hand.is_none());
                assert!(weapons.get(key).unwrap().id().is_some() == should_have_id);
                assert_eq!(
                    weapons.get(key).unwrap().tags(),
                    [
                        WeaponTag::Lethal,
                        WeaponTag::Melee,
                        WeaponTag::OneHanded,
                        WeaponTag::Thrown(RangeBand::Short),
                        WeaponTag::Light
                    ]
                    .into()
                );
                assert_eq!(
                    weapons.get(key).unwrap().data_source(),
                    &DataSource::Book(BookReference {
                        book_title: "Core Rulebook".to_owned(),
                        page_number: 581,
                    })
                );
            }
            "Screamer (Red Jade Reaper Daiklave)" => {
                assert!(maybe_hand == Some(EquipHand::Main));
                assert!(weapons.get(key).unwrap().id().is_some() == should_have_id);
                assert_eq!(
                    weapons.get(key).unwrap().tags(),
                    [
                        WeaponTag::Lethal,
                        WeaponTag::Melee,
                        WeaponTag::OneHanded,
                        WeaponTag::Medium,
                        WeaponTag::Balanced,
                        WeaponTag::Artifact,
                        WeaponTag::MartialArts("Single Point Shining Into Void Style".to_owned())
                    ]
                    .into()
                );
                if should_have_id {
                    assert!(match weapons.get(key).unwrap().data_source() {
                        DataSource::Book(_) => panic!("should be custom"),
                        DataSource::Custom(None) => panic!("should have custom creator id"),
                        DataSource::Custom(Some(_)) => true,
                    });
                } else {
                    assert_eq!(
                        weapons.get(key).unwrap().data_source(),
                        &DataSource::Custom(None)
                    );
                }
            }
            wrong => panic!("Unknown armor name: {}", wrong),
        }
    }
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
    let initial_character = create_initial_character(&receive_player);
    validate_initial_base_character(&player, &initial_character, false);

    assert_eq!(initial_character.experience, initial_character.experience);
    assert_eq!(
        initial_character
            .attributes
            .iter()
            .map(|attr| (attr.name(), attr.dots()))
            .collect::<HashMap<AttributeName, u8>>(),
        vec![
            (AttributeName::Strength, 4),
            (AttributeName::Dexterity, 4),
            (AttributeName::Stamina, 3),
            (AttributeName::Charisma, 4),
            (AttributeName::Manipulation, 3),
            (AttributeName::Appearance, 2),
            (AttributeName::Intelligence, 3),
            (AttributeName::Wits, 3),
            (AttributeName::Perception, 1)
        ]
        .into_iter()
        .collect::<HashMap::<AttributeName, u8>>()
    );
    check_initial_abilities(&initial_character.abilities);
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
    assert_eq!(initial_character.health.damage(), (2, 3, 1));
    assert_eq!(
        initial_character
            .health
            .health_boxes()
            .iter()
            .map(|hbox| { (hbox.wound_penalty(), hbox.damage()) })
            .collect::<Vec<_>>(),
        vec![
            (WoundPenalty::Zero, DamageLevel::Aggravated),
            (WoundPenalty::MinusOne, DamageLevel::Lethal),
            (WoundPenalty::MinusOne, DamageLevel::Lethal),
            (WoundPenalty::MinusOne, DamageLevel::Lethal),
            (WoundPenalty::MinusTwo, DamageLevel::Bashing),
            (WoundPenalty::MinusTwo, DamageLevel::Bashing),
            (WoundPenalty::MinusTwo, DamageLevel::None),
            (WoundPenalty::MinusTwo, DamageLevel::None),
            (WoundPenalty::MinusFour, DamageLevel::None),
            (WoundPenalty::Incapacitated, DamageLevel::None)
        ]
    );
    check_initial_armor_items(&initial_character.armor, false);
    check_initial_weapons(&initial_character.weapons, false);

    // Client builds, serializes, and sends to server
    let send_bytes = postcard::to_allocvec(&initial_character).unwrap();

    // Server deserializes character
    let receive_character: Character = from_bytes(&send_bytes).unwrap();
    validate_deserialization(&initial_character, &receive_character);
    check_initial_abilities(&receive_character.abilities);
    assert!(receive_character
        .intimacies
        .iter()
        .all(|i| i.id().is_none()));
    check_initial_armor_items(&receive_character.armor, false);
    check_initial_weapons(&receive_character.weapons, false);

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
    check_initial_abilities(&post_insert_character.abilities);
    assert!(post_insert_character
        .intimacies
        .iter()
        .all(|i| i.id().is_some()));
    assert_eq!(&receive_character.health, &post_insert_character.health);
    check_initial_armor_items(&post_insert_character.armor, true);
    check_initial_weapons(&post_insert_character.weapons, true);

    // Server serializes and sends character to client
    let send_bytes = postcard::to_allocvec(&post_insert_character).unwrap();

    // Client deserializes character and modifies it
    let fetched_character: Character = from_bytes(&send_bytes).unwrap();
    validate_deserialization(&initial_character, &receive_character);
    check_initial_abilities(&fetched_character.abilities);
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
    check_initial_armor_items(&fetched_character.armor, true);
    check_initial_weapons(&fetched_character.weapons, true);

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
