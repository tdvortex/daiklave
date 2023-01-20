use daiklave_core2::{CharacterEventSource, CharacterMutation};

#[test]
fn test_name_and_concept() {
    // Check default name and concept
    let mut event_source = CharacterEventSource::default();
    let character = event_source.as_character().unwrap();
    assert_eq!(character.name(), "New Character");
    assert!(character.concept().is_none());

    // Check set name
    let mutation = CharacterMutation::SetName("Drifting Leaves".to_owned());
    let character = event_source.apply_mutation(mutation).unwrap();
    assert_eq!(character.name(), "Drifting Leaves");

    // Check set concept
    let mutation = CharacterMutation::SetConcept("Wandering ronin".to_owned());
    let character = event_source.apply_mutation(mutation).unwrap();
    assert_eq!(character.concept(), Some("Wandering ronin"));

    // Check remove concept
    let mutation = CharacterMutation::RemoveConcept;
    let character = event_source.apply_mutation(mutation).unwrap();
    assert!(character.concept().is_none());

    // Check that we can undo the full history
    assert!(!event_source.can_redo());
    assert!(event_source.can_undo());
    let character = event_source.undo().unwrap();
    assert_eq!(character.concept(), Some("Wandering ronin"));

    assert!(event_source.can_redo());
    assert!(event_source.can_undo());
    let character = event_source.undo().unwrap();
    assert_eq!(character.name(), "Drifting Leaves");

    assert!(event_source.can_redo());
    assert!(event_source.can_undo());
    let character = event_source.undo().unwrap();
    assert_eq!(character.name(), "New Character");
    assert!(character.concept().is_none());

    assert!(!event_source.can_undo());

    // Check we can redo the full history
    let character = event_source.redo().unwrap();
    assert_eq!(character.name(), "Drifting Leaves");

    let character = event_source.redo().unwrap();
    assert_eq!(character.concept(), Some("Wandering ronin"));

    let character = event_source.redo().unwrap();
    assert!(character.concept().is_none());
}
