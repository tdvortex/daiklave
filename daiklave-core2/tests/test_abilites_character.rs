use daiklave_core2::{Character, abilities::AbilityNameVanilla};

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
