use exalted_3e_gui::{
    character::{CharacterBuilder, ExperiencePoints, Willpower},
    player::Player,
    Character,
};

pub fn create_initial_base_character(player: &Player) -> CharacterBuilder {
    Character::create()
        .with_player(player.clone())
        .with_name("Test Character Name".to_owned())
        .with_concept("A character for testing purposes".to_owned())
        .with_willpower(Willpower {
            current: 5,
            maximum: 6,
        })
        .with_experience(ExperiencePoints {
            current: 15,
            total: 15,
        })
}

pub fn validate_initial_base_character(
    player: &Player,
    character: &Character,
    should_have_id: bool,
) {
    assert_eq!(character.id().is_some(), should_have_id);
    assert_eq!(character.player(), player);
    assert_eq!(character.name.as_str(), "Test Character Name");
    assert!(character.campaign().is_none());
    assert_eq!(
        character.concept.as_deref(),
        Some("A character for testing purposes")
    );
    assert_eq!(character.willpower.current, 5);
    assert_eq!(character.willpower.maximum, 6);
    assert_eq!(character.experience.current, 15);
    assert_eq!(character.experience.total, 15);
}

pub fn modify_base_character(character: &mut Character) {
    // Erase character concept
    character.concept = None;

    // Give him a better name
    character.name = "Incomparable Wanderer".to_owned();

    // Update willpower
    character.willpower.maximum = 7;
    character.willpower.recover_all();

    // Update experience
    character.experience.total += 5;
    character.experience.current -= 5;
}

pub fn validate_modified_base_character(player: &Player, character: &Character) {
    assert_eq!(character.player(), player);
    assert_eq!(character.name.as_str(), "Incomparable Wanderer");
    assert!(character.campaign().is_none());
    assert!(character.concept.is_none());
    assert_eq!(character.willpower.current, 7);
    assert_eq!(character.willpower.maximum, 7);
    assert_eq!(character.experience.current, 10);
    assert_eq!(character.experience.total, 20);
}
