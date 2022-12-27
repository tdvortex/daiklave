use daiklave_core2::{Character, CharacterEventSource, CharacterMutation, CharacterView};

#[test]
fn test_name_and_concept_character() {
    // Check default name and concept
    let mut character = Character::default();
    assert_eq!(character.name(), "New Character");
    assert!(character.concept().is_none());

    // Check set name
    assert!(character.check_set_name("Drifting Leaves").is_ok());
    assert!(character.set_name("Drifting Leaves").is_ok());
    assert_eq!(character.name(), "Drifting Leaves");

    // Check set concept
    assert!(character.check_set_concept("Wandering ronin").is_ok());
    assert!(character.set_concept("Wandering ronin").is_ok());
    assert_eq!(character.concept(), Some("Wandering ronin"));

    // Check remove concept
    assert!(character.check_remove_concept().is_ok());
    assert!(character.remove_concept().is_ok());
    assert!(character.concept().is_none());
}

#[test]
fn test_name_and_concept_character_view() {
    // Check default name and concept
    let mut character_view = CharacterView::default();
    assert_eq!(character_view.name(), "New Character");
    assert!(character_view.concept().is_none());

    // Check set name
    assert!(character_view.check_set_name("Drifting Leaves").is_ok());
    assert!(character_view.set_name("Drifting Leaves").is_ok());
    assert_eq!(character_view.name(), "Drifting Leaves");

    // Check set concept
    assert!(character_view.check_set_concept("Wandering ronin").is_ok());
    assert!(character_view.set_concept("Wandering ronin").is_ok());
    assert_eq!(character_view.concept(), Some("Wandering ronin"));

    // Check remove concept
    assert!(character_view.check_remove_concept().is_ok());
    assert!(character_view.remove_concept().is_ok());
    assert!(character_view.concept().is_none());
}

#[test]
fn test_name_and_concept_character_event_source() {
    // Check default name and concept
    let mut event_source = CharacterEventSource::default();
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.name(), "New Character");
    assert!(character_view.concept().is_none());

    // Check set name
    let mutation = CharacterMutation::SetName("Drifting Leaves".to_owned());
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.name(), "Drifting Leaves");

    // Check set concept
    let mutation = CharacterMutation::SetConcept("Wandering ronin".to_owned());
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.concept(), Some("Wandering ronin"));

    // Check remove concept
    let mutation = CharacterMutation::RemoveConcept;
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    assert!(character_view.concept().is_none());

    // Check that we can undo the full history
    assert!(!event_source.can_redo());
    assert!(event_source.can_undo());
    assert!(event_source.undo());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.concept(), Some("Wandering ronin"));

    assert!(event_source.can_redo());
    assert!(event_source.can_undo());
    assert!(event_source.undo());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.name(), "Drifting Leaves");

    assert!(event_source.can_redo());
    assert!(event_source.can_undo());
    assert!(event_source.undo());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.name(), "New Character");
    assert!(character_view.concept().is_none());

    assert!(!event_source.can_undo());

    // Check we can redo the full history
    assert!(event_source.redo());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.name(), "Drifting Leaves");

    assert!(event_source.redo());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.concept(), Some("Wandering ronin"));

    assert!(event_source.redo());
    let character_view = event_source.as_character_view().unwrap();
    assert!(character_view.concept().is_none());
}
