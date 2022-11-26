use std::collections::HashSet;

use exalted_3e_gui::{AbilityName, MortalCharacter};

fn default_mortal_character() -> MortalCharacter {
    MortalCharacter::default()
}

fn custom_mortal_character() -> MortalCharacter {
    let mut mortal = MortalCharacter::default();
    mortal
        .abilities
        .add_martial_arts_style("Crane Style".to_owned());

    [
        (AbilityName::Athletics, 3),
        (AbilityName::Brawl, 1),
        (AbilityName::Bureaucracy, 2),
        (AbilityName::Integrity, 3),
        (AbilityName::Investigation, 2),
        (AbilityName::Larcency, 2),
        (AbilityName::Linguistics, 3),
        (AbilityName::Lore, 2),
        (AbilityName::MartialArts("Crane Style".to_owned()), 4),
        (AbilityName::Occult, 2),
        (AbilityName::Presence, 5),
        (AbilityName::Resistance, 2),
        (AbilityName::Socialize, 5),
        (AbilityName::Thrown, 1),
    ]
    .into_iter()
    .for_each(|(ability_name, rating)| {
        mortal
            .abilities
            .get_mut(&ability_name)
            .unwrap()
            .set_dots(rating);
    });

    [
        (AbilityName::Bureaucracy, "Realm Bureaucratic Processes"),
        (AbilityName::Bureaucracy, "Management"),
        (AbilityName::Socialize, "Formal Situations"),
        (AbilityName::Presence, "Asserting Authority"),
        (AbilityName::Presence, "Convincing Authorities"),
    ]
    .into_iter()
    .for_each(|(ability_name, specialty)| {
        mortal
            .abilities
            .get_mut(&ability_name)
            .unwrap()
            .add_specialty(specialty.to_owned())
            .unwrap();
    });

    mortal
}

#[test]
fn test_build_default() {
    let _mortal = default_mortal_character();
}

#[test]
fn test_build_custom() {
    let _mortal = custom_mortal_character();
}

#[test]
fn test_default_abilities() {
    let mortal = default_mortal_character();

    let actual: Vec<(AbilityName, u8, Option<Vec<String>>)> = mortal
        .abilities
        .iter()
        .map(|ability| {
            (
                ability.name().clone(),
                ability.dots().clone(),
                ability
                    .specialties()
                    .map(|s| s.iter().map(|s| s.clone()).collect()),
            )
        })
        .collect();

    let expected: Vec<(AbilityName, u8, Option<Vec<String>>)> = vec![
        (AbilityName::Archery, 0, None),
        (AbilityName::Athletics, 0, None),
        (AbilityName::Awareness, 0, None),
        (AbilityName::Brawl, 0, None),
        (AbilityName::Bureaucracy, 0, None),
        (AbilityName::Dodge, 0, None),
        (AbilityName::Integrity, 0, None),
        (AbilityName::Investigation, 0, None),
        (AbilityName::Larcency, 0, None),
        (AbilityName::Linguistics, 0, None),
        (AbilityName::Lore, 0, None),
        (AbilityName::Medicine, 0, None),
        (AbilityName::Melee, 0, None),
        (AbilityName::Occult, 0, None),
        (AbilityName::Performance, 0, None),
        (AbilityName::Presence, 0, None),
        (AbilityName::Resistance, 0, None),
        (AbilityName::Ride, 0, None),
        (AbilityName::Sail, 0, None),
        (AbilityName::Socialize, 0, None),
        (AbilityName::Stealth, 0, None),
        (AbilityName::Survival, 0, None),
        (AbilityName::Thrown, 0, None),
        (AbilityName::War, 0, None),
    ];

    for (act, exp) in actual.into_iter().zip(expected.into_iter()) {
        assert_eq!(act, exp)
    }
}

#[test]
fn test_custom_abilities() {
    let mortal = custom_mortal_character();

    let actual: Vec<(AbilityName, u8, Option<HashSet<String>>)> = mortal
        .abilities
        .iter()
        .map(|ability| {
            (
                ability.name().clone(),
                ability.dots().clone(),
                ability
                    .specialties()
                    .map(|s| s.iter().map(|s| s.clone()).collect()),
            )
        })
        .collect();

    let expected: Vec<(AbilityName, u8, Option<HashSet<String>>)> = vec![
        (AbilityName::Archery, 0, None),
        (AbilityName::Athletics, 3, None),
        (AbilityName::Awareness, 0, None),
        (AbilityName::Brawl, 1, None),
        (
            AbilityName::Bureaucracy,
            2,
            Some(
                [
                    "Realm Bureaucratic Processes".to_owned(),
                    "Management".to_owned(),
                ]
                .into_iter()
                .collect(),
            ),
        ),
        (AbilityName::Dodge, 0, None),
        (AbilityName::Integrity, 3, None),
        (AbilityName::Investigation, 2, None),
        (AbilityName::Larcency, 2, None),
        (AbilityName::Linguistics, 3, None),
        (AbilityName::Lore, 2, None),
        (AbilityName::Medicine, 0, None),
        (AbilityName::Melee, 0, None),
        (AbilityName::Occult, 2, None),
        (AbilityName::Performance, 0, None),
        (
            AbilityName::Presence,
            5,
            Some(
                [
                    "Asserting Authority".to_owned(),
                    "Convincing Authorities".to_owned(),
                ]
                .into_iter()
                .collect(),
            ),
        ),
        (AbilityName::Resistance, 2, None),
        (AbilityName::Ride, 0, None),
        (AbilityName::Sail, 0, None),
        (
            AbilityName::Socialize,
            5,
            Some(["Formal Situations".to_owned()].into_iter().collect()),
        ),
        (AbilityName::Stealth, 0, None),
        (AbilityName::Survival, 0, None),
        (AbilityName::Thrown, 1, None),
        (AbilityName::War, 0, None),
        (AbilityName::MartialArts("Crane Style".to_owned()), 4, None),
    ];

    for (act, exp) in actual.into_iter().zip(expected.into_iter()) {
        assert_eq!(act, exp)
    }
}
