use exalted_3e_gui::{character::CharacterBuilder, abilities::{AbilityNameNoSubskill, Abilities}};

pub fn create_intitial_abilities(builder: CharacterBuilder) -> CharacterBuilder {
    vec![
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
    .fold(builder, |ic, (ability_name_no_subskill, dots)| {
        ic.with_ability(ability_name_no_subskill, dots).unwrap()
    })
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
}

pub fn validate_initial_abilities(abilities: &Abilities) {
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