use daiklave_core2::{abilities::AbilityNameVanilla, CharacterEventSource, CharacterMutation};

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
            character_view.abilities().get(*ability_name_vanilla).dots(),
            *expected_dots
        );
        assert_eq!(
            character_view
                .abilities()
                .get(*ability_name_vanilla)
                .specialties()
                .next(),
            *expected_specialties
        );
    }

    // Check set ability dots
    let mutation = CharacterMutation::SetAbilityDots(AbilityNameVanilla::Archery, 1);
    character_view.check_mutation(&mutation).unwrap();
    event_source.apply_mutation(mutation).unwrap();
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(
        character_view
            .abilities()
            .get(AbilityNameVanilla::Archery)
            .dots(),
        1
    );

    // Check add specialty
    let mutation =
        CharacterMutation::AddSpecialty(AbilityNameVanilla::Archery, "Firewands".to_owned());
    character_view.check_mutation(&mutation).unwrap();
    event_source.apply_mutation(mutation).unwrap();
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(
        character_view
            .abilities()
            .get(AbilityNameVanilla::Archery)
            .specialties()
            .next(),
        Some("Firewands")
    );

    // Check remove specialty
    let mutation =
        CharacterMutation::RemoveSpecialty(AbilityNameVanilla::Archery, "Firewands".to_owned());
    character_view.check_mutation(&mutation).unwrap();
    event_source.apply_mutation(mutation).unwrap();
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(
        character_view
            .abilities()
            .get(AbilityNameVanilla::Archery)
            .specialties()
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
            .get(AbilityNameVanilla::Archery)
            .specialties()
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
