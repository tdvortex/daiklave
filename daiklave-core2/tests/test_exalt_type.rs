use daiklave_core2::{
    abilities::AbilityName,
    exaltation::exalt::exalt_type::solar::{caste::twilight::Twilight, Solar},
    Character, CharacterEventSource, CharacterMutation,
};

#[test]
fn test_exalt_type_character_view() {
    // Confirm default is mortal
    let mut character_view = Character::default();
    assert!(character_view.is_mortal());

    // Confirm toggle to solar
    let twilight = {
        let mut builder = Twilight::builder();
        [
            AbilityName::Bureaucracy,
            AbilityName::Craft,
            AbilityName::Integrity,
            AbilityName::Investigation,
        ]
        .into_iter()
        .for_each(|ability| {
            builder.add_caste_ability(ability).unwrap();
        });
        builder
            .set_supernal_ability(AbilityName::Linguistics)
            .unwrap();
        builder.build().unwrap()
    };

    let solar_traits = {
        let mut builder = Solar::builder();
        builder.set_twilight(twilight);
        [
            AbilityName::Archery,
            AbilityName::Athletics,
            AbilityName::Awareness,
            AbilityName::Brawl,
            AbilityName::Dodge,
        ]
        .into_iter()
        .for_each(|ability| {
            builder.add_favored_ability(ability).unwrap();
        });
        builder.build_view().unwrap().as_memo()
    };

    character_view.check_set_solar(&solar_traits).unwrap();
    character_view.set_solar(&solar_traits).unwrap();
    assert!(character_view.is_solar());

    // Confirm toggle to mortal
    character_view.check_set_mortal().unwrap();
    character_view.set_mortal().unwrap();
    assert!(character_view.is_mortal());
}

#[test]
fn test_exalt_type_character_event_source() {
    // Check default is mortal
    let mut event_source = CharacterEventSource::default();
    let character_view = event_source.as_character_view().unwrap();
    assert!(character_view.is_mortal());

    // Check toggle to solar
    let twilight = {
        let mut builder = Twilight::builder();
        [
            AbilityName::Bureaucracy,
            AbilityName::Craft,
            AbilityName::Integrity,
            AbilityName::Investigation,
        ]
        .into_iter()
        .for_each(|ability| {
            builder.add_caste_ability(ability).unwrap();
        });
        builder
            .set_supernal_ability(AbilityName::Linguistics)
            .unwrap();
        builder.build().unwrap()
    };

    let solar_traits = {
        let mut builder = Solar::builder();
        builder.set_twilight(twilight);
        [
            AbilityName::Archery,
            AbilityName::Athletics,
            AbilityName::Awareness,
            AbilityName::Brawl,
            AbilityName::Dodge,
        ]
        .into_iter()
        .for_each(|ability| {
            builder.add_favored_ability(ability).unwrap();
        });
        builder.build_view().unwrap().as_memo()
    };

    let mutation = CharacterMutation::SetSolar(Box::new(solar_traits));
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
