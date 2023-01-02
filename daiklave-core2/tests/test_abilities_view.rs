use daiklave_core2::{abilities::AbilityNameVanilla, Character};

#[test]
fn test_abilities_character_view() {
    // Check default abilities
    let mut character_view = Character::default();
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
    character_view
        .check_set_ability_dots(AbilityNameVanilla::Archery, 1)
        .unwrap();
    character_view
        .set_ability_dots(AbilityNameVanilla::Archery, 1)
        .unwrap();
    assert_eq!(
        character_view
            .abilities()
            .get(AbilityNameVanilla::Archery)
            .dots(),
        1
    );

    // Check add specialty
    character_view
        .check_add_specialty(AbilityNameVanilla::Archery, "Firewands")
        .unwrap();
    character_view
        .add_specialty(AbilityNameVanilla::Archery, "Firewands")
        .unwrap();
    assert_eq!(
        character_view
            .abilities()
            .get(AbilityNameVanilla::Archery)
            .specialties()
            .next(),
        Some("Firewands")
    );

    // Check remove specialty
    character_view
        .check_remove_specialty(AbilityNameVanilla::Archery, "Firewands")
        .unwrap();
    character_view
        .remove_specialty(AbilityNameVanilla::Archery, "Firewands")
        .unwrap();
    assert_eq!(
        character_view
            .abilities()
            .get(AbilityNameVanilla::Archery)
            .specialties()
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
            .get(AbilityNameVanilla::Archery)
            .specialties()
            .next(),
        None
    );
}
