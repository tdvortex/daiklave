use daiklave_core2::{
    AbilityNameVanilla, Character, CharacterEventSource, CharacterMutation, CharacterView,
};

#[test]
fn test_abilities_character() {
    // Check default abilities
    let mut character = Character::default();
    let expected: Vec<(AbilityNameVanilla, u8, Option<&str>)> = vec![
        (AbilityNameVanilla::Archery, 0, None),
        (AbilityNameVanilla::Athletics, 0, None),
        (AbilityNameVanilla::Awareness, 0, None),
        (AbilityNameVanilla::Brawl, 0, None),
        (AbilityNameVanilla::Bureaucracy, 0, None),
        (AbilityNameVanilla::Dodge, 0, None),
        (AbilityNameVanilla::Integrity, 0, None),
        (AbilityNameVanilla::Investigation, 0, None),
        (AbilityNameVanilla::Larceny, 0, None),
        (AbilityNameVanilla::Linguistics, 0, None),
        (AbilityNameVanilla::Lore, 0, None),
        (AbilityNameVanilla::Medicine, 0, None),
        (AbilityNameVanilla::Melee, 0, None),
        (AbilityNameVanilla::Occult, 0, None),
        (AbilityNameVanilla::Performance, 0, None),
        (AbilityNameVanilla::Presence, 0, None),
        (AbilityNameVanilla::Resistance, 0, None),
        (AbilityNameVanilla::Ride, 0, None),
        (AbilityNameVanilla::Sail, 0, None),
        (AbilityNameVanilla::Socialize, 0, None),
        (AbilityNameVanilla::Stealth, 0, None),
        (AbilityNameVanilla::Survival, 0, None),
        (AbilityNameVanilla::Thrown, 0, None),
        (AbilityNameVanilla::War, 0, None),
    ];

    for (ability_name_vanilla, expected_dots, expected_specialties) in expected.iter() {
        assert_eq!(
            character.abilities().dots(*ability_name_vanilla),
            *expected_dots
        );
        assert_eq!(
            character
                .abilities()
                .specialties(*ability_name_vanilla)
                .next(),
            *expected_specialties
        );
    }

    // Check set ability dots
    assert!(character
        .check_set_ability_dots(AbilityNameVanilla::Archery, 1)
        .is_ok());
    assert!(character
        .set_ability_dots(AbilityNameVanilla::Archery, 1)
        .is_ok());
    assert_eq!(character.abilities().dots(AbilityNameVanilla::Archery), 1);

    // Check add specialty
    assert!(character
        .check_add_specialty(AbilityNameVanilla::Archery, "Firewands")
        .is_ok());
    assert!(character
        .add_specialty(AbilityNameVanilla::Archery, "Firewands")
        .is_ok());
    assert_eq!(
        character
            .abilities()
            .specialties(AbilityNameVanilla::Archery)
            .next(),
        Some("Firewands")
    );

    // Check remove specialty
    assert!(character
        .check_remove_specialty(AbilityNameVanilla::Archery, "Firewands")
        .is_ok());
    assert!(character
        .remove_specialty(AbilityNameVanilla::Archery, "Firewands")
        .is_ok());
    assert_eq!(
        character
            .abilities()
            .specialties(AbilityNameVanilla::Archery)
            .next(),
        None
    );

    // Check can't add specialties to zero-rated abilities
    assert!(character
        .check_add_specialty(AbilityNameVanilla::Athletics, "Bad specialty")
        .is_err());

    // Check can't remove nonexistent specialties
    assert!(character
        .check_remove_specialty(AbilityNameVanilla::Athletics, "Bad specialty")
        .is_err());

    // Check can't add duplicate specialties
    character
        .add_specialty(AbilityNameVanilla::Archery, "Firewands")
        .unwrap();
    assert!(character
        .check_add_specialty(AbilityNameVanilla::Archery, "Firewands")
        .is_err());

    // Check setting an ability to zero removes all specialties
    character
        .set_ability_dots(AbilityNameVanilla::Archery, 0)
        .unwrap();
    assert_eq!(
        character
            .abilities()
            .specialties(AbilityNameVanilla::Archery)
            .next(),
        None
    );
}

#[test]
fn test_abilities_character_view() {
    // Check default abilities
    let mut character_view = CharacterView::default();
    let expected: Vec<(AbilityNameVanilla, u8, Option<&str>)> = vec![
        (AbilityNameVanilla::Archery, 0, None),
        (AbilityNameVanilla::Athletics, 0, None),
        (AbilityNameVanilla::Awareness, 0, None),
        (AbilityNameVanilla::Brawl, 0, None),
        (AbilityNameVanilla::Bureaucracy, 0, None),
        (AbilityNameVanilla::Dodge, 0, None),
        (AbilityNameVanilla::Integrity, 0, None),
        (AbilityNameVanilla::Investigation, 0, None),
        (AbilityNameVanilla::Larceny, 0, None),
        (AbilityNameVanilla::Linguistics, 0, None),
        (AbilityNameVanilla::Lore, 0, None),
        (AbilityNameVanilla::Medicine, 0, None),
        (AbilityNameVanilla::Melee, 0, None),
        (AbilityNameVanilla::Occult, 0, None),
        (AbilityNameVanilla::Performance, 0, None),
        (AbilityNameVanilla::Presence, 0, None),
        (AbilityNameVanilla::Resistance, 0, None),
        (AbilityNameVanilla::Ride, 0, None),
        (AbilityNameVanilla::Sail, 0, None),
        (AbilityNameVanilla::Socialize, 0, None),
        (AbilityNameVanilla::Stealth, 0, None),
        (AbilityNameVanilla::Survival, 0, None),
        (AbilityNameVanilla::Thrown, 0, None),
        (AbilityNameVanilla::War, 0, None),
    ];

    for (ability_name_vanilla, expected_dots, expected_specialties) in expected.iter() {
        assert_eq!(
            character_view.abilities().dots(*ability_name_vanilla),
            *expected_dots
        );
        assert_eq!(
            character_view
                .abilities()
                .specialties(*ability_name_vanilla)
                .next(),
            *expected_specialties
        );
    }

    // Check set ability dots
    assert!(character_view
        .check_set_ability_dots(AbilityNameVanilla::Archery, 1)
        .is_ok());
    assert!(character_view
        .set_ability_dots(AbilityNameVanilla::Archery, 1)
        .is_ok());
    assert_eq!(
        character_view.abilities().dots(AbilityNameVanilla::Archery),
        1
    );

    // Check add specialty
    assert!(character_view
        .check_add_specialty(AbilityNameVanilla::Archery, "Firewands")
        .is_ok());
    assert!(character_view
        .add_specialty(AbilityNameVanilla::Archery, "Firewands")
        .is_ok());
    assert_eq!(
        character_view
            .abilities()
            .specialties(AbilityNameVanilla::Archery)
            .next(),
        Some("Firewands")
    );

    // Check remove specialty
    assert!(character_view
        .check_remove_specialty(AbilityNameVanilla::Archery, "Firewands")
        .is_ok());
    assert!(character_view
        .remove_specialty(AbilityNameVanilla::Archery, "Firewands")
        .is_ok());
    assert_eq!(
        character_view
            .abilities()
            .specialties(AbilityNameVanilla::Archery)
            .next(),
        None
    );

    // Check can't add specialties to zero-rated abilities
    assert!(character_view
        .check_add_specialty(AbilityNameVanilla::Athletics, "Bad specialty")
        .is_err());

    // Check can't remove nonexistent specialties
    assert!(character_view
        .check_remove_specialty(AbilityNameVanilla::Athletics, "Bad specialty")
        .is_err());

    // Check can't add duplicate specialties
    character_view
        .add_specialty(AbilityNameVanilla::Archery, "Firewands")
        .unwrap();
    assert!(character_view
        .check_add_specialty(AbilityNameVanilla::Archery, "Firewands")
        .is_err());

    // Check setting an ability to zero removes all specialties
    character_view
        .set_ability_dots(AbilityNameVanilla::Archery, 0)
        .unwrap();
    assert_eq!(
        character_view
            .abilities()
            .specialties(AbilityNameVanilla::Archery)
            .next(),
        None
    );
}

#[test]
fn test_abilities_character_event_source() {
    // Check default abilities
    let mut event_source = CharacterEventSource::default();
    let character_view = event_source.as_character_view().unwrap();
    let expected: Vec<(AbilityNameVanilla, u8, Option<&str>)> = vec![
        (AbilityNameVanilla::Archery, 0, None),
        (AbilityNameVanilla::Athletics, 0, None),
        (AbilityNameVanilla::Awareness, 0, None),
        (AbilityNameVanilla::Brawl, 0, None),
        (AbilityNameVanilla::Bureaucracy, 0, None),
        (AbilityNameVanilla::Dodge, 0, None),
        (AbilityNameVanilla::Integrity, 0, None),
        (AbilityNameVanilla::Investigation, 0, None),
        (AbilityNameVanilla::Larceny, 0, None),
        (AbilityNameVanilla::Linguistics, 0, None),
        (AbilityNameVanilla::Lore, 0, None),
        (AbilityNameVanilla::Medicine, 0, None),
        (AbilityNameVanilla::Melee, 0, None),
        (AbilityNameVanilla::Occult, 0, None),
        (AbilityNameVanilla::Performance, 0, None),
        (AbilityNameVanilla::Presence, 0, None),
        (AbilityNameVanilla::Resistance, 0, None),
        (AbilityNameVanilla::Ride, 0, None),
        (AbilityNameVanilla::Sail, 0, None),
        (AbilityNameVanilla::Socialize, 0, None),
        (AbilityNameVanilla::Stealth, 0, None),
        (AbilityNameVanilla::Survival, 0, None),
        (AbilityNameVanilla::Thrown, 0, None),
        (AbilityNameVanilla::War, 0, None),
    ];

    for (ability_name_vanilla, expected_dots, expected_specialties) in expected.iter() {
        assert_eq!(
            character_view.abilities().dots(*ability_name_vanilla),
            *expected_dots
        );
        assert_eq!(
            character_view
                .abilities()
                .specialties(*ability_name_vanilla)
                .next(),
            *expected_specialties
        );
    }

    // Check set ability dots
    let mutation = CharacterMutation::SetAbilityDots(AbilityNameVanilla::Archery, 1);
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(
        character_view.abilities().dots(AbilityNameVanilla::Archery),
        1
    );

    // Check add specialty
    let mutation =
        CharacterMutation::AddSpecialty(AbilityNameVanilla::Archery, "Firewands".to_owned());
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(
        character_view
            .abilities()
            .specialties(AbilityNameVanilla::Archery)
            .next(),
        Some("Firewands")
    );

    // Check remove specialty
    let mutation =
        CharacterMutation::RemoveSpecialty(AbilityNameVanilla::Archery, "Firewands".to_owned());
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(
        character_view
            .abilities()
            .specialties(AbilityNameVanilla::Archery)
            .next(),
        None
    );

    // Check can't add specialties to zero-rated abilities
    let mutation =
        CharacterMutation::AddSpecialty(AbilityNameVanilla::Athletics, "Bad specialty".to_owned());
    assert!(character_view.check_mutation(&mutation).is_err());

    // Check can't remove nonexistent specialties
    let mutation = CharacterMutation::RemoveSpecialty(
        AbilityNameVanilla::Athletics,
        "Bad specialty".to_owned(),
    );
    assert!(character_view.check_mutation(&mutation).is_err());

    // Check can't add duplicate specialties
    let mutation =
        CharacterMutation::AddSpecialty(AbilityNameVanilla::Archery, "Firewands".to_owned());
    event_source.apply_mutation(mutation.clone()).unwrap();
    let character_view = event_source.as_character_view().unwrap();
    assert!(character_view.check_mutation(&mutation).is_err());

    // Check setting an ability to zero removes all specialties
    let mutation = CharacterMutation::SetAbilityDots(AbilityNameVanilla::Archery, 0);
    event_source.apply_mutation(mutation).unwrap();
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(
        character_view
            .abilities()
            .specialties(AbilityNameVanilla::Archery)
            .next(),
        None
    );

    // Check we can undo the full history
    assert!(!event_source.can_redo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(!event_source.can_undo());

    // Check we can redo the full history
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(!event_source.can_redo());
}
