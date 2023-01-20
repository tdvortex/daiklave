use daiklave_core2::{
    abilities::AbilityName,
    exaltation::exalt::exalt_type::solar::{
        caste::{DawnCasteAbility, DawnSupernalAbility},
        Solar,
    },
    CharacterEventSource, CharacterMutation,
};

#[test]
fn test_willpower_character_event_source() {
    // Check default (mortal)
    let mut event_source = CharacterEventSource::default();
    let character_view = event_source.as_character().unwrap();
    assert_eq!(character_view.willpower().rating(), 3);
    assert_eq!(character_view.willpower().current(), 3);

    // Check default (exalt)
    let new_solar = Solar::builder()
        .dawn()
        .caste_ability(DawnCasteAbility::Dodge)
        .caste_ability(DawnCasteAbility::Resistance)
        .caste_ability(DawnCasteAbility::Awareness)
        .caste_ability(DawnCasteAbility::War)
        .supernal_ability(DawnSupernalAbility::MartialArts)
        .favored_ability(AbilityName::Presence)
        .favored_ability(AbilityName::Socialize)
        .favored_ability(AbilityName::Linguistics)
        .favored_ability(AbilityName::Medicine)
        .favored_ability(AbilityName::Performance)
        .limit_trigger("Fleeing from a battle not yet lost".to_owned())
        .build()
        .unwrap();
    let mutation = CharacterMutation::SetSolar(new_solar);
    event_source.apply_mutation(mutation).unwrap();
    let character_view = event_source.as_character().unwrap();
    assert_eq!(character_view.willpower().rating(), 5);
    assert_eq!(character_view.willpower().current(), 5);

    // Check modifying current willpower
    let mutation = CharacterMutation::SetCurrentWillpower(3);
    let character = event_source.apply_mutation(mutation).unwrap();
    assert_eq!(character.willpower().rating(), 5);
    assert_eq!(character.willpower().current(), 3);

    // Check modifying willpower rating
    let mutation = CharacterMutation::SetWillpowerRating(7);
    let character = event_source.apply_mutation(mutation).unwrap();
    assert_eq!(character.willpower().rating(), 7);
    assert_eq!(character.willpower().current(), 7);

    // Check we can undo the full history
    assert!(!event_source.can_redo());
    assert!(event_source.undo().is_ok());
    assert!(event_source.undo().is_ok());
    assert!(event_source.undo().is_ok());
    assert!(!event_source.can_undo());

    // Check we can redo the full history
    assert!(event_source.redo().is_ok());
    assert!(event_source.redo().is_ok());
    assert!(event_source.redo().is_ok());
    assert!(!event_source.can_redo());
}
