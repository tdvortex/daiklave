use daiklave_core2::{
    abilities::AbilityName,
    exaltation::exalt::exalt_type::solar::{Solar},
    CharacterEventSource, CharacterMutation,
};

#[test]
fn test_exalt_type() {
    // Check default is mortal
    let mut event_source = CharacterEventSource::default();
    let character_view = event_source.as_character_view().unwrap();
    assert!(character_view.is_mortal());

    // Confirm toggle to solar
    let add_solar = Solar::builder()
        .twilight()
        .caste_ability(TwilightAbility::Bureaucracy)
        .caste_ability(TwilightAbility::Craft)
        .caste_ability(TwilightAbility::Integrity)
        .caste_ability(TwilightAbility::Investigation)
        .caste_ability(TwilightAbility::Linguistics)
        .supernal_ability(TwilightAbility::Linguistics)
        .favored_ability(AbilityName::Archery)
        .favored_ability(AbilityName::Athletics)
        .favored_ability(AbilityName::Awareness)
        .favored_ability(AbilityName::Brawl)
        .favored_ability(AbilityName::Dodge)
        .limit_trigger("Being told you're wrong when you're not".to_owned())
        .build()
        .unwrap();

    let mutation = CharacterMutation::SetSolar(add_solar);
    character_view.check_mutation(&mutation).unwrap();
    event_source.apply_mutation(mutation).unwrap();
    let character_view = event_source.as_character_view().unwrap();
    assert!(character_view.is_solar());

    // Check toggle to mortal
    let mutation = CharacterMutation::SetMortal;
    character_view.check_mutation(&mutation).unwrap();
    event_source.apply_mutation(mutation).unwrap();
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
