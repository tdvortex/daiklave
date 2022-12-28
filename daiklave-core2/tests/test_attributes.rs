use daiklave_core2::{AttributeName, Character, CharacterView};

#[test]
fn test_attributes_character() {
    // Check default attributes
    let mut character = Character::default();
    assert_eq!(character.attributes().get_dots(AttributeName::Strength), 1);
    assert_eq!(character.attributes().get_dots(AttributeName::Dexterity), 1);
    assert_eq!(character.attributes().get_dots(AttributeName::Stamina), 1);
    assert_eq!(character.attributes().get_dots(AttributeName::Charisma), 1);
    assert_eq!(
        character.attributes().get_dots(AttributeName::Manipulation),
        1
    );
    assert_eq!(
        character.attributes().get_dots(AttributeName::Appearance),
        1
    );
    assert_eq!(
        character.attributes().get_dots(AttributeName::Perception),
        1
    );
    assert_eq!(
        character.attributes().get_dots(AttributeName::Intelligence),
        1
    );
    assert_eq!(character.attributes().get_dots(AttributeName::Wits), 1);

    // Check setting attributes
    assert!(character
        .check_set_attribute(AttributeName::Strength, 2)
        .is_ok());
    assert!(character.set_attribute(AttributeName::Strength, 2).is_ok());
    assert_eq!(character.attributes().get_dots(AttributeName::Strength), 2);

    // Check out-of-bounds prevention
    assert!(character
        .check_set_attribute(AttributeName::Dexterity, 0)
        .is_err());
    assert!(character
        .check_set_attribute(AttributeName::Dexterity, 6)
        .is_err());
}

#[test]
fn test_attributes_character_view() {
    // Check default attributes
    let mut character_view = CharacterView::default();
    assert_eq!(
        character_view
            .attributes()
            .get_dots(AttributeName::Strength),
        1
    );
    assert_eq!(
        character_view
            .attributes()
            .get_dots(AttributeName::Dexterity),
        1
    );
    assert_eq!(
        character_view.attributes().get_dots(AttributeName::Stamina),
        1
    );
    assert_eq!(
        character_view
            .attributes()
            .get_dots(AttributeName::Charisma),
        1
    );
    assert_eq!(
        character_view
            .attributes()
            .get_dots(AttributeName::Manipulation),
        1
    );
    assert_eq!(
        character_view
            .attributes()
            .get_dots(AttributeName::Appearance),
        1
    );
    assert_eq!(
        character_view
            .attributes()
            .get_dots(AttributeName::Perception),
        1
    );
    assert_eq!(
        character_view
            .attributes()
            .get_dots(AttributeName::Intelligence),
        1
    );
    assert_eq!(character_view.attributes().get_dots(AttributeName::Wits), 1);

    // Check setting attributes
    assert!(character_view
        .check_set_attribute(AttributeName::Strength, 2)
        .is_ok());
    assert!(character_view
        .set_attribute(AttributeName::Strength, 2)
        .is_ok());
    assert_eq!(
        character_view
            .attributes()
            .get_dots(AttributeName::Strength),
        2
    );

    // Check out-of-bounds prevention
    assert!(character_view
        .check_set_attribute(AttributeName::Dexterity, 0)
        .is_err());
    assert!(character_view
        .check_set_attribute(AttributeName::Dexterity, 6)
        .is_err());
}
