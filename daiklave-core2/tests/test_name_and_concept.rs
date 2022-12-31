use daiklave_core2::{Character, CharacterEventSource, CharacterMutation, CharacterView};

#[test]
fn test_name_and_concept_character() {
    // Check default name and concept
    let mut character = Character::default();
    assert_eq!(character.name(), "New Character");
    assert!(character.concept().is_none());

    // Check set name
    character.check_set_name("Drifting Leaves").unwrap();
    character.set_name("Drifting Leaves").unwrap();
    assert_eq!(character.name(), "Drifting Leaves");

    // Check set concept
    character.check_set_concept("Wandering ronin").unwrap();
    character.set_concept("Wandering ronin").unwrap();
    assert_eq!(character.concept(), Some("Wandering ronin"));

    // Check remove concept
    character.check_remove_concept().unwrap();
    character.remove_concept().unwrap();
    assert!(character.concept().is_none());
}

#[test]
fn test_name_and_concept_character_view() {
    // Check default name and concept
    let mut character_view = CharacterView::default();
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
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.name(), "New Character");
    assert!(character_view.concept().is_none());

    // Check set name
    let mutation = CharacterMutation::SetName("Drifting Leaves".to_owned());
    character_view.check_mutation(&mutation).unwrap();
    event_source.apply_mutation(mutation).unwrap();
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.name(), "Drifting Leaves");

    // Check set concept
    let mutation = CharacterMutation::SetConcept("Wandering ronin".to_owned());
    character_view.check_mutation(&mutation).unwrap();
    event_source.apply_mutation(mutation).unwrap();
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.concept(), Some("Wandering ronin"));

    // Check remove concept
    let mutation = CharacterMutation::RemoveConcept;
    character_view.check_mutation(&mutation).unwrap();
    event_source.apply_mutation(mutation).unwrap();
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
