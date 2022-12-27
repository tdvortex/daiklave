use daiklave_core2::{Character, id::{CharacterId, Id}, CharacterView, CharacterEventSource, CharacterMutation};

#[test]
fn test_set_id_character() {
    // Check default Id
    let mut character = Character::default();
    assert_eq!(character.id(), CharacterId(Id::Placeholder(0)));

    // Check that we can override with placeholder Id
    let new_placeholder = CharacterId(Id::Placeholder(1));
    assert!(character.check_set_id(new_placeholder).is_ok());
    character.set_id(new_placeholder).unwrap();
    assert_eq!(character.id(), CharacterId(Id::Placeholder(1)));

    // Check that trying to override a placeholder Id with NonUnique fails
    let non_unique = CharacterId(Id::NonUnique);
    assert!(character.check_set_id(non_unique).is_err());
    assert!(character.set_id(non_unique).is_err());
    assert_eq!(character.id(), CharacterId(Id::Placeholder(1)));

    // Check that we can override with database Id
    let new_database = CharacterId(Id::Database(2));
    assert!(character.check_set_id(new_database).is_ok());
    character.set_id(new_database).unwrap();
    assert_eq!(character.id(), CharacterId(Id::Database(2)));

    // Check that we cannot override database Id
    let err_database = CharacterId(Id::Database(3));
    assert!(character.check_set_id(err_database).is_err());
    assert!(character.set_id(err_database).is_err());
    assert_eq!(character.id(), CharacterId(Id::Database(2)));
}

#[test]
fn test_set_id_character_view() {
    // Check default Id
    let mut character_view = CharacterView::default();
    assert_eq!(character_view.id(), CharacterId(Id::Placeholder(0)));

    // Check that we can override with placeholder Id
    let new_placeholder = CharacterId(Id::Placeholder(1));
    assert!(character_view.check_set_id(new_placeholder).is_ok());
    character_view.set_id(new_placeholder).unwrap();
    assert_eq!(character_view.id(), CharacterId(Id::Placeholder(1)));

    // Check that trying to override a placeholder Id with NonUnique fails
    let non_unique = CharacterId(Id::NonUnique);
    assert!(character_view.check_set_id(non_unique).is_err());
    assert!(character_view.set_id(non_unique).is_err());
    assert_eq!(character_view.id(), CharacterId(Id::Placeholder(1)));

    // Check that we can override with database Id
    let new_database = CharacterId(Id::Database(2));
    assert!(character_view.check_set_id(new_database).is_ok());
    character_view.set_id(new_database).unwrap();
    assert_eq!(character_view.id(), CharacterId(Id::Database(2)));

    // Check that we cannot override database Id
    let err_database = CharacterId(Id::Database(3));
    assert!(character_view.check_set_id(err_database).is_err());
    assert!(character_view.set_id(err_database).is_err());
    assert_eq!(character_view.id(), CharacterId(Id::Database(2)));
}

#[test]
fn test_set_id_character_event_source() {
    // Check default Id
    let mut event_source = CharacterEventSource::default();
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.id(), CharacterId(Id::Placeholder(0)));

    // Check that we can override with placeholder Id
    let mutation = CharacterMutation::SetId(CharacterId(Id::Placeholder(1)));
    assert!(character_view.check_mutation(&mutation).is_ok());
    event_source.apply_mutation(mutation).unwrap();
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.id(), CharacterId(Id::Placeholder(1)));

    // Check that trying to override a placeholder Id with NonUnique fails
    let mutation = CharacterMutation::SetId(CharacterId(Id::NonUnique));
    assert!(character_view.check_mutation(&mutation).is_err());
    assert!(event_source.apply_mutation(mutation).is_err());

    // Check that we can override with database Id
    let mutation = CharacterMutation::SetId(CharacterId(Id::Database(2)));
    assert!(character_view.check_mutation(&mutation).is_ok());
    event_source.apply_mutation(mutation).unwrap();
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.id(), CharacterId(Id::Database(2)));

    // Check that we cannot override database Id
    let mutation = CharacterMutation::SetId(CharacterId(Id::Database(3)));
    assert!(character_view.check_mutation(&mutation).is_err());
    assert!(event_source.apply_mutation(mutation).is_err());
    assert_eq!(character_view.id(), CharacterId(Id::Database(2)));

    // Check that invalid unchecked cannot be reconstructed
    let mutation = CharacterMutation::SetId(CharacterId(Id::Database(3)));
    event_source.apply_mutation_unchecked(mutation);
    assert!(event_source.as_character_view().is_err());

    // Check that undoing an invalid operation restores proper behavior
    assert!(!event_source.can_redo());
    assert!(event_source.can_undo());
    assert!(event_source.undo());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.id(), CharacterId(Id::Database(2)));

    // Check we can undo the full history
    assert!(event_source.can_redo());
    assert!(event_source.can_undo());
    assert!(event_source.undo());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.id(), CharacterId(Id::Placeholder(1)));

    assert!(event_source.can_redo());
    assert!(event_source.can_undo());
    assert!(event_source.undo());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.id(), CharacterId(Id::Placeholder(0)));

    assert!(event_source.can_redo());
    assert!(!event_source.can_undo());

    // Check we can redo the full history
    assert!(event_source.redo());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.id(), CharacterId(Id::Placeholder(1)));

    assert!(event_source.redo());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.id(), CharacterId(Id::Database(2)));

    assert!(event_source.redo());
    assert!(event_source.as_character_view().is_err());
}