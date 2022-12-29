use daiklave_core2::{
    Character, CharacterEventSource, CharacterMutation, CharacterView, Solar,
};

#[test]
fn test_willpower_character() {
    // Check default (mortal)
    let mut character = Character::default();
    assert_eq!(character.willpower().rating(), 3);
    assert_eq!(character.willpower().current(), 3);

    // Check default (exalt)
    let solar_traits = Solar::builder().build();
    character.set_solar(&solar_traits).unwrap();
    assert_eq!(character.willpower().rating(), 5);
    assert_eq!(character.willpower().current(), 5);

    // Check modifying current willpower
    assert!(character.check_set_current_willpower(3).is_ok());
    assert!(character.set_current_willpower(3).is_ok());
    assert_eq!(character.willpower().rating(), 5);
    assert_eq!(character.willpower().current(), 3);

    // Check modifying willpower rating
    assert!(character.check_set_willpower_rating(7).is_ok());
    assert!(character.set_willpower_rating(7).is_ok());
    assert_eq!(character.willpower().rating(), 7);
    assert_eq!(character.willpower().current(), 7);
}

#[test]
fn test_willpower_character_view() {
    // Check default (mortal)
    let mut character_view = CharacterView::default();
    assert_eq!(character_view.willpower().rating(), 3);
    assert_eq!(character_view.willpower().current(), 3);

    // Check default (exalt)
    let solar_traits = Solar::builder().build();
    character_view.set_solar(&solar_traits).unwrap();
    assert_eq!(character_view.willpower().rating(), 5);
    assert_eq!(character_view.willpower().current(), 5);

    // Check modifying current willpower
    assert!(character_view.check_set_current_willpower(3).is_ok());
    assert!(character_view.set_current_willpower(3).is_ok());
    assert_eq!(character_view.willpower().rating(), 5);
    assert_eq!(character_view.willpower().current(), 3);

    // Check modifying willpower rating
    assert!(character_view.check_set_willpower_rating(7).is_ok());
    assert!(character_view.set_willpower_rating(7).is_ok());
    assert_eq!(character_view.willpower().rating(), 7);
    assert_eq!(character_view.willpower().current(), 7);
}

#[test]
fn test_willpower_character_event_source() {
    // Check default (mortal)
    let mut event_source = CharacterEventSource::default();
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.willpower().rating(), 3);
    assert_eq!(character_view.willpower().current(), 3);

    // Check default (exalt)
    let solar_traits = Solar::builder().build();
    let mutation = CharacterMutation::SetSolar(solar_traits);
    event_source.apply_mutation(mutation).unwrap();
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.willpower().rating(), 5);
    assert_eq!(character_view.willpower().current(), 5);

    // Check modifying current willpower
    let mutation = CharacterMutation::SetCurrentWillpower(3);
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.willpower().rating(), 5);
    assert_eq!(character_view.willpower().current(), 3);

    // Check modifying willpower rating
    let mutation = CharacterMutation::SetWillpowerRating(7);
    assert!(character_view.check_mutation(&mutation).is_ok());
    event_source.apply_mutation(mutation).unwrap();
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.willpower().rating(), 7);
    assert_eq!(character_view.willpower().current(), 7);

    // Check we can undo the full history
    assert!(!event_source.can_redo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(!event_source.can_undo());

    // Check we can redo the full history
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(!event_source.can_redo());
}
