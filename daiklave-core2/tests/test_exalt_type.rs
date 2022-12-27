use daiklave_core2::{
    Character, CharacterEventSource, CharacterMutation, CharacterView, SolarTraits,
};

#[test]
fn test_exalt_type_character() {
    // Confirm default is mortal
    let mut character = Character::default();
    assert!(character.is_mortal());

    // Confirm toggle to solar
    let solar_traits = SolarTraits::builder().build();
    assert!(character.check_set_solar(&solar_traits).is_ok());
    assert!(character.set_solar(&solar_traits).is_ok());
    assert!(character.is_solar());

    // Confirm toggle to mortal
    assert!(character.check_set_mortal().is_ok());
    assert!(character.set_mortal().is_ok());
    assert!(character.is_mortal());
}

#[test]
fn test_exalt_type_character_view() {
    // Confirm default is mortal
    let mut character_view = CharacterView::default();
    assert!(character_view.is_mortal());

    // Confirm toggle to solar
    let solar_traits = SolarTraits::builder().build();
    assert!(character_view.check_set_solar(&solar_traits).is_ok());
    assert!(character_view.set_solar(&solar_traits).is_ok());
    assert!(character_view.is_solar());

    // Confirm toggle to mortal
    assert!(character_view.check_set_mortal().is_ok());
    assert!(character_view.set_mortal().is_ok());
    assert!(character_view.is_mortal());
}

#[test]
fn test_exalt_type_character_event_source() {
    // Check default is mortal
    let mut event_source = CharacterEventSource::default();
    let character_view = event_source.as_character_view().unwrap();
    assert!(character_view.is_mortal());

    // Check toggle to solar
    let solar_traits = SolarTraits::builder().build();
    let mutation = CharacterMutation::SetSolar(solar_traits);
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    assert!(character_view.is_solar());

    // Check toggle to mortal
    let mutation = CharacterMutation::SetMortal;
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    assert!(character_view.is_mortal());

    // Check we can undo full history
    assert!(!event_source.can_redo());
    assert!(event_source.can_undo());
    assert!(event_source.undo());
    let character_view = event_source.as_character_view().unwrap();
    assert!(character_view.is_solar());

    assert!(event_source.can_redo());
    assert!(event_source.can_undo());
    assert!(event_source.undo());
    let character_view = event_source.as_character_view().unwrap();
    assert!(character_view.is_mortal());

    assert!(event_source.can_redo());
    assert!(!event_source.can_undo());

    // Check we can redo full history
    assert!(event_source.redo());
    let character_view = event_source.as_character_view().unwrap();
    assert!(character_view.is_solar());

    assert!(event_source.redo());
    let character_view = event_source.as_character_view().unwrap();
    assert!(character_view.is_mortal());
}
