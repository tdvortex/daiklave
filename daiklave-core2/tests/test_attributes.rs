use daiklave_core2::{attributes::AttributeName, CharacterEventSource, CharacterMutation};

#[test]
fn test_attributes() {
    // Check default attributes
    let mut event_source = CharacterEventSource::default();
    let character = event_source.as_character().unwrap();
    let attributes = character.attributes();
    [
        AttributeName::Strength,
        AttributeName::Dexterity,
        AttributeName::Stamina,
        AttributeName::Charisma,
        AttributeName::Manipulation,
        AttributeName::Appearance,
        AttributeName::Intelligence,
        AttributeName::Perception,
        AttributeName::Wits,
    ]
    .into_iter()
    .for_each(|name| assert_eq!(attributes.get(name).dots(), 1));

    // Check setting attributes
    let mutation = CharacterMutation::SetAttribute(AttributeName::Strength, 2);
    let character = event_source.apply_mutation(mutation).unwrap();

    assert_eq!(
        character.attributes().get(AttributeName::Strength).dots(),
        2
    );

    // Check out-of-bounds prevention
    let mutation = CharacterMutation::SetAttribute(AttributeName::Dexterity, 0);
    assert!(event_source.apply_mutation(mutation).is_err());
    let mutation = CharacterMutation::SetAttribute(AttributeName::Dexterity, 6);
    assert!(event_source.apply_mutation(mutation).is_err());

    // Check undo and redo
    assert!(!event_source.can_redo());
    let character = event_source.undo().unwrap();
    assert_eq!(
        character.attributes().get(AttributeName::Strength).dots(),
        1
    );
    assert!(!event_source.can_undo());
    let character = event_source.redo().unwrap();
    assert_eq!(
        character.attributes().get(AttributeName::Strength).dots(),
        2
    );
}
