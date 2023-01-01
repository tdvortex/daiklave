use daiklave_core2::{
    attributes::AttributeName, CharacterMemo, CharacterEventSource, CharacterMutation, CharacterView,
};

#[test]
fn test_attributes_character() {
    // Check default attributes
    let mut character = CharacterMemo::default();
    assert_eq!(character.attributes().dots(AttributeName::Strength), 1);
    assert_eq!(character.attributes().dots(AttributeName::Dexterity), 1);
    assert_eq!(character.attributes().dots(AttributeName::Stamina), 1);
    assert_eq!(character.attributes().dots(AttributeName::Charisma), 1);
    assert_eq!(character.attributes().dots(AttributeName::Manipulation), 1);
    assert_eq!(character.attributes().dots(AttributeName::Appearance), 1);
    assert_eq!(character.attributes().dots(AttributeName::Perception), 1);
    assert_eq!(character.attributes().dots(AttributeName::Intelligence), 1);
    assert_eq!(character.attributes().dots(AttributeName::Wits), 1);

    // Check setting attributes
    character
        .check_set_attribute(AttributeName::Strength, 2)
        .unwrap();
    character.set_attribute(AttributeName::Strength, 2).unwrap();
    assert_eq!(character.attributes().dots(AttributeName::Strength), 2);

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
    assert_eq!(character_view.attributes().dots(AttributeName::Strength), 1);
    assert_eq!(
        character_view.attributes().dots(AttributeName::Dexterity),
        1
    );
    assert_eq!(character_view.attributes().dots(AttributeName::Stamina), 1);
    assert_eq!(character_view.attributes().dots(AttributeName::Charisma), 1);
    assert_eq!(
        character_view
            .attributes()
            .dots(AttributeName::Manipulation),
        1
    );
    assert_eq!(
        character_view.attributes().dots(AttributeName::Appearance),
        1
    );
    assert_eq!(
        character_view.attributes().dots(AttributeName::Perception),
        1
    );
    assert_eq!(
        character_view
            .attributes()
            .dots(AttributeName::Intelligence),
        1
    );
    assert_eq!(character_view.attributes().dots(AttributeName::Wits), 1);

    // Check setting attributes
    character_view
        .check_set_attribute(AttributeName::Strength, 2)
        .unwrap();
    character_view
        .set_attribute(AttributeName::Strength, 2)
        .unwrap();
    assert_eq!(character_view.attributes().dots(AttributeName::Strength), 2);

    // Check out-of-bounds prevention
    assert!(character_view
        .check_set_attribute(AttributeName::Dexterity, 0)
        .is_err());
    assert!(character_view
        .check_set_attribute(AttributeName::Dexterity, 6)
        .is_err());
}

#[test]
fn test_attributes_character_event_source() {
    // Check default attributes
    let mut event_source = CharacterEventSource::default();
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.attributes().dots(AttributeName::Strength), 1);
    assert_eq!(
        character_view.attributes().dots(AttributeName::Dexterity),
        1
    );
    assert_eq!(character_view.attributes().dots(AttributeName::Stamina), 1);
    assert_eq!(character_view.attributes().dots(AttributeName::Charisma), 1);
    assert_eq!(
        character_view
            .attributes()
            .dots(AttributeName::Manipulation),
        1
    );
    assert_eq!(
        character_view.attributes().dots(AttributeName::Appearance),
        1
    );
    assert_eq!(
        character_view.attributes().dots(AttributeName::Perception),
        1
    );
    assert_eq!(
        character_view
            .attributes()
            .dots(AttributeName::Intelligence),
        1
    );
    assert_eq!(character_view.attributes().dots(AttributeName::Wits), 1);

    // Check setting attributes
    let mutation = CharacterMutation::SetAttribute(AttributeName::Strength, 2);
    character_view.check_mutation(&mutation).unwrap();
    event_source.apply_mutation(mutation).unwrap();
    let character_view = event_source.as_character_view().unwrap();

    assert_eq!(character_view.attributes().dots(AttributeName::Strength), 2);

    // Check out-of-bounds prevention
    assert!(character_view
        .check_set_attribute(AttributeName::Dexterity, 0)
        .is_err());
    assert!(character_view
        .check_set_attribute(AttributeName::Dexterity, 6)
        .is_err());

    // Check undo and redo
    assert!(!event_source.can_redo());
    assert!(event_source.undo());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.attributes().dots(AttributeName::Strength), 1);
    assert!(!event_source.can_undo());
    assert!(event_source.redo());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.attributes().dots(AttributeName::Strength), 2);
}
