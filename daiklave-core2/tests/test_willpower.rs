use daiklave_core2::{Character, abilities::AbilityName, exalt_state::exalt::exalt_type::solar::{Dawn, Solar}, CharacterView, CharacterEventSource, CharacterMutation};

#[test]
fn test_willpower_character() {
    // Check default (mortal)
    let mut character = Character::default();
    assert_eq!(character.willpower().rating(), 3);
    assert_eq!(character.willpower().current(), 3);

    // Check default (exalt)
    let dawn = {
        let mut builder = Dawn::builder();
        [
            AbilityName::Dodge,
            AbilityName::Resistance,
            AbilityName::Awareness,
            AbilityName::War,
        ]
        .into_iter()
        .for_each(|ability| {
            builder.add_caste_ability(ability).unwrap();
        });
        builder
            .set_supernal_ability(AbilityName::MartialArts)
            .unwrap();
        builder.build().unwrap()
    };

    let solar_traits = {
        let mut builder = Solar::builder();
        builder.set_dawn(dawn);
        [
            AbilityName::Presence,
            AbilityName::Socialize,
            AbilityName::Linguistics,
            AbilityName::Medicine,
            AbilityName::Performance,
        ]
        .into_iter()
        .for_each(|ability| {
            builder.add_favored_ability(ability).unwrap();
        });
        builder.build().unwrap()
    };

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
    let dawn = {
        let mut builder = Dawn::builder();
        [
            AbilityName::Dodge,
            AbilityName::Resistance,
            AbilityName::Awareness,
            AbilityName::War,
        ]
        .into_iter()
        .for_each(|ability| {
            builder.add_caste_ability(ability).unwrap();
        });
        builder
            .set_supernal_ability(AbilityName::MartialArts)
            .unwrap();
        builder.build().unwrap()
    };

    let solar_traits = {
        let mut builder = Solar::builder();
        builder.set_dawn(dawn);
        [
            AbilityName::Presence,
            AbilityName::Socialize,
            AbilityName::Linguistics,
            AbilityName::Medicine,
            AbilityName::Performance,
        ]
        .into_iter()
        .for_each(|ability| {
            builder.add_favored_ability(ability).unwrap();
        });
        builder.build().unwrap()
    };

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
    let dawn = {
        let mut builder = Dawn::builder();
        [
            AbilityName::Dodge,
            AbilityName::Resistance,
            AbilityName::Awareness,
            AbilityName::War,
        ]
        .into_iter()
        .for_each(|ability| {
            builder.add_caste_ability(ability).unwrap();
        });
        builder
            .set_supernal_ability(AbilityName::MartialArts)
            .unwrap();
        builder.build().unwrap()
    };

    let solar_traits = {
        let mut builder = Solar::builder();
        builder.set_dawn(dawn);
        [
            AbilityName::Presence,
            AbilityName::Socialize,
            AbilityName::Linguistics,
            AbilityName::Medicine,
            AbilityName::Performance,
        ]
        .into_iter()
        .for_each(|ability| {
            builder.add_favored_ability(ability).unwrap();
        });
        builder.build().unwrap()
    };

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
