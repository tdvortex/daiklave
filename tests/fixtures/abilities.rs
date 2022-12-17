use exalted_3e_gui::{
    abilities::AbilityNameNoSubskill, character::CharacterBuilder, id::Id,
    martial_arts::MartialArtsStyle, Character,
};

const SINGLE_POINT_STYLE_DESCRIPTION: &str =
    "Single Point Shining Into the Void is a sword style that \
    emphasizes blinding speed and deadly-perfect finishing \
    moves. Students learn to draw their blade as though it \
    were an extension of their own body, transitioning \
    effortlessly from the draw to a strike or parry. When a \
    master duels a lesser swordsman, the fight often ends in \
    a single stroke, the blade sheathed almost in the same \
    instant it's drawn. When faced against a foe who matches \
    her in skill, the Single Point stylist fights with swift blows \
    and an almost meditative focus, seeking the single moment \
    of weakness in her foe's defenses that will allow her to \
    slash through him entirely. \n\
    Single Point Shining Into the Void Weapons: This style \
    uses slashing swords and their artifact equivalents, reaper \
    daiklaves, delivering lightning-fast attacks from the draw. \
    It cannot be used unarmed. \n \
    Armor: This style is compatible with light and medium
    armor.";

pub fn create_intitial_abilities(builder: CharacterBuilder) -> CharacterBuilder {
    let single_point_shining_into_the_void_style =
        MartialArtsStyle::from_book(Id::Placeholder(0), "Core Rulebook".to_owned(), 434)
            .with_name("Single Point Shining Into the Void Style".to_owned())
            .with_description(SINGLE_POINT_STYLE_DESCRIPTION.to_owned())
            .build()
            .unwrap();

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
    .with_martial_arts_style(single_point_shining_into_the_void_style, 4)
    .unwrap()
    .with_specialty(AbilityNameNoSubskill::War, "While Outnumbered".to_owned())
    .unwrap()
    .with_specialty(AbilityNameNoSubskill::Socialize, "Tavern Gossip".to_owned())
    .unwrap()
    .with_craft_specialty("Weapon Forging", "Sharpening Blades".to_owned())
    .unwrap()
    .with_martial_arts_specialty(Id::Placeholder(0), "Join Battle".to_owned())
    .unwrap()
}

pub fn validate_initial_abilities(character: &Character) {
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
            Some("Single Point Shining Into the Void Style"),
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
                character
                    .get_ability(ability_name_no_subskill, subskill)
                    .unwrap()
                    .dots(),
                expect_dots
            );
            assert_eq!(
                character
                    .get_ability(ability_name_no_subskill, subskill)
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
        assert!(character
            .get_ability(ability_name_no_subskill, subskill)
            .is_none());
    });
}

pub fn modify_abilities(character: &mut Character) {
    // Increase a stable ability
    character
        .set_ability_dots(AbilityNameNoSubskill::Dodge, None, 4)
        .unwrap();
    // Decrease a stable ability with specialty to zero
    character
        .set_ability_dots(AbilityNameNoSubskill::Socialize, None, 0)
        .unwrap();
    // Add a new subskilled ability
    character
        .set_ability_dots(AbilityNameNoSubskill::Craft, Some("Origami"), 1)
        .unwrap();
    // Increase an existing subskilled ability
    character
        .set_ability_dots(
            AbilityNameNoSubskill::MartialArts,
            Some("Single Point Shining Into the Void Style"),
            5,
        )
        .unwrap();
    // Decrease an existing subskilled ability with specialty to zero
    character
        .set_ability_dots(AbilityNameNoSubskill::Craft, Some("Weapon Forging"), 0)
        .unwrap();
    // Add a specialty
    character
        .add_specialty(
            AbilityNameNoSubskill::Integrity,
            None,
            "Patience".to_owned(),
        )
        .unwrap();
    // Remove a specialty
    character
        .remove_specialty(AbilityNameNoSubskill::War, None, "While Outnumbered")
        .unwrap();
}

pub fn validate_modified_abilities(character: &Character) {
    vec![
        (AbilityNameNoSubskill::Archery, None, 0, None),
        (AbilityNameNoSubskill::Athletics, None, 2, None),
        (AbilityNameNoSubskill::Awareness, None, 4, None),
        (AbilityNameNoSubskill::Brawl, None, 1, None),
        (AbilityNameNoSubskill::Bureaucracy, None, 0, None),
        (AbilityNameNoSubskill::Craft, Some("Origami"), 1, None),
        (AbilityNameNoSubskill::Dodge, None, 4, None),
        (
            AbilityNameNoSubskill::Integrity,
            None,
            2,
            Some(&(["Patience".to_owned()].into())),
        ),
        (AbilityNameNoSubskill::Investigation, None, 0, None),
        (AbilityNameNoSubskill::Larceny, None, 0, None),
        (AbilityNameNoSubskill::Linguistics, None, 1, None),
        (AbilityNameNoSubskill::Lore, None, 0, None),
        (
            AbilityNameNoSubskill::MartialArts,
            Some("Single Point Shining Into the Void Style"),
            5,
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
        (AbilityNameNoSubskill::Socialize, None, 0, None),
        (AbilityNameNoSubskill::Stealth, None, 0, None),
        (AbilityNameNoSubskill::Survival, None, 0, None),
        (AbilityNameNoSubskill::Thrown, None, 0, None),
        (AbilityNameNoSubskill::War, None, 3, None),
    ]
    .into_iter()
    .for_each(
        |(ability_name_no_subskill, subskill, expect_dots, expect_specialties)| {
            assert_eq!(
                character
                    .get_ability(ability_name_no_subskill, subskill)
                    .unwrap()
                    .dots(),
                expect_dots
            );
            assert_eq!(
                character
                    .get_ability(ability_name_no_subskill, subskill)
                    .unwrap()
                    .specialties(),
                expect_specialties
            );
        },
    );

    vec![
        (AbilityNameNoSubskill::Craft, Some("Weapon Forging")),
        (AbilityNameNoSubskill::MartialArts, Some("Does Not Exist")),
    ]
    .into_iter()
    .for_each(|(ability_name_no_subskill, subskill)| {
        assert!(character
            .get_ability(ability_name_no_subskill, subskill)
            .is_none());
    });
}
