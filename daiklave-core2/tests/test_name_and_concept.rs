use daiklave_core2::{Character, CharacterEventSource, CharacterMutation};

#[test]
fn test_name_and_concept_character_view() {
    // Check default name and concept
    let mut character_view = Character::default();
    assert_eq!(character_view.name(), "New Character");
    assert!(character_view.concept().is_none());

    // Check set name
    character_view.check_set_name("Drifting Leaves").unwrap();
    character_view.set_name("Drifting Leaves").unwrap();
    assert_eq!(character_view.name(), "Drifting Leaves");

    // Check set concept
    character_view.check_set_concept("Wandering ronin").unwrap();
    character_view.set_concept("Wandering ronin").unwrap();
    assert_eq!(character_view.concept(), Some("Wandering ronin"));

    // Check remove concept
    character_view.check_remove_concept().unwrap();
    character_view.remove_concept().unwrap();
    assert!(character_view.concept().is_none());
}

#[test]
fn test_name_and_concept_character_event_source() {
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
