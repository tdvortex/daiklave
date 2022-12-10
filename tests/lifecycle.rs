use std::collections::{HashMap, HashSet};

use exalted_3e_gui::{
    abilities::{Abilities, AbilityNameNoSubskill},
    attributes::AttributeName,
    character::{ExperiencePoints, Willpower},
    create_player, destroy_player,
    health::{DamageLevel, WoundPenalty},
    intimacies::{Intimacy, IntimacyLevel, IntimacyType},
    player::Player,
    update_character, Character,
};
use postcard::from_bytes;
use sqlx::postgres::PgPool;

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
    let initial_character = {
        let mut initial_character = Character::create()
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
            });

        initial_character = vec![
            (AttributeName::Strength, 4),
            (AttributeName::Dexterity, 4),
            (AttributeName::Stamina, 3),
            (AttributeName::Charisma, 4),
            (AttributeName::Manipulation, 3),
            (AttributeName::Appearance, 2),
            (AttributeName::Intelligence, 3),
            (AttributeName::Wits, 3),
        ]
        .into_iter()
        .fold(initial_character, |ic, (attribute_name, value)| {
            ic.with_attribute(attribute_name, value).unwrap()
        });

        initial_character = vec![
            (AbilityNameNoSubskill::Awareness, 4),
            (AbilityNameNoSubskill::War, 3),
            (AbilityNameNoSubskill::Resistance, 3),
            (AbilityNameNoSubskill::Dodge, 3),
            (AbilityNameNoSubskill::Integrity, 2),
            (AbilityNameNoSubskill::Presence, 2),
            (AbilityNameNoSubskill::Socialize, 2),
            (AbilityNameNoSubskill::Athletics, 2),
            (AbilityNameNoSubskill::Linguistics, 1),
            (AbilityNameNoSubskill::Brawl, 1),
        ]
        .into_iter()
        .fold(initial_character, |ic, (ability_name_no_subskill, dots)| {
            ic.with_ability(ability_name_no_subskill, dots).unwrap()
        });

        initial_character
            .with_craft("Weapon Forging", 1)
            .with_martial_arts("Single Point Shining Into Void Style", 4)
            .with_specialty(AbilityNameNoSubskill::War, "While Outnumbered".to_owned())
            .unwrap()
            .with_specialty(AbilityNameNoSubskill::Socialize, "Tavern Gossip".to_owned())
            .unwrap()
            .with_craft_specialty("Weapon Forging", "Sharpening Blades".to_owned())
            .unwrap()
            .with_martial_arts_specialty(
                "Single Point Shining Into Void Style",
                "Join Battle".to_owned(),
            )
            .unwrap()
            .with_intimacy(Intimacy::new(
                IntimacyLevel::Defining,
                IntimacyType::Principle,
                "Never stand idle against injustice".to_owned(),
                None,
            ))
            .with_intimacy(Intimacy::new(
                IntimacyLevel::Major,
                IntimacyType::Tie,
                "Ragara Tirnis (Love)".to_owned(),
                None,
            ))
            .with_intimacy(Intimacy::new(
                IntimacyLevel::Major,
                IntimacyType::Tie,
                "Mask of Winters (Revenge)".to_owned(),
                None,
            ))
            .with_intimacy(Intimacy::new(
                IntimacyLevel::Minor,
                IntimacyType::Tie,
                "Street Vendors (Camaraderie)".to_owned(),
                None,
            ))
            .with_wound_penalties(vec![
                WoundPenalty::Incapacitated,
                WoundPenalty::MinusFour,
                WoundPenalty::MinusTwo,
                WoundPenalty::MinusTwo,
                WoundPenalty::MinusTwo,
                WoundPenalty::MinusTwo,
                WoundPenalty::MinusOne,
                WoundPenalty::MinusOne,
                WoundPenalty::MinusOne,
                WoundPenalty::Zero,
            ])
            .with_damage(2, 3, 1)
            .build()
            .unwrap()
    };

    assert!(initial_character.id().is_none());
    assert_eq!(initial_character.player(), &receive_player);
    assert_eq!(&initial_character.name, "Test Character Name");
    assert_eq!(
        initial_character.concept.as_deref(),
        Some("A character for testing purposes")
    );
    assert_eq!(
        initial_character.willpower,
        Willpower {
            current: 5,
            maximum: 6,
        }
    );
    assert_eq!(
        initial_character.experience,
        ExperiencePoints {
            current: 15,
            total: 15,
        }
    );
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

    // Client builds, serializes, and sends to server
    let send_bytes = postcard::to_allocvec(&initial_character).unwrap();

    // Server deserializes character
    let receive_character: Character = from_bytes(&send_bytes).unwrap();
    assert!(receive_character.id().is_none());
    assert_eq!(receive_character.player(), &receive_player);
    assert_eq!(receive_character.name, initial_character.name);
    assert_eq!(receive_character.concept, initial_character.concept);
    assert_eq!(receive_character.willpower, initial_character.willpower);
    assert_eq!(receive_character.experience, initial_character.experience);
    check_initial_abilities(&receive_character.abilities);
    check_intimacies_except_id(&receive_character.intimacies, &initial_character.intimacies);
    assert!(receive_character
        .intimacies
        .iter()
        .all(|i| i.id().is_none()));
    assert_eq!(&receive_character.health, &initial_character.health);

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

    // Server serializes and sends character to client
    let send_bytes = postcard::to_allocvec(&post_insert_character).unwrap();

    // Client deserializes character and modifies it
    let fetched_character: Character = from_bytes(&send_bytes).unwrap();
    assert_eq!(fetched_character.id(), post_insert_character.id());
    assert_eq!(fetched_character.player(), post_insert_character.player());
    assert_eq!(fetched_character.name, post_insert_character.name);
    assert_eq!(fetched_character.concept, post_insert_character.concept);
    assert_eq!(fetched_character.willpower, post_insert_character.willpower);
    assert_eq!(
        fetched_character.experience,
        post_insert_character.experience
    );
    assert_eq!(
        fetched_character.attributes,
        post_insert_character.attributes
    );
    check_initial_abilities(&fetched_character.abilities);
    check_intimacies_except_id(
        &fetched_character.intimacies,
        &post_insert_character.intimacies,
    );
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
    assert_eq!(&fetched_character.health, &post_insert_character.health);

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

    // Clean up database to end test
}
