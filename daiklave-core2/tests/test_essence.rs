use daiklave_core2::{
    abilities::AbilityName,
    exaltation::exalt::{
        essence::{MoteCommitmentId, MotePoolName, UncommitMotes},
        exalt_type::solar::{caste::EclipseAbility, Solar},
    },
    CharacterEventSource, CharacterMutation,
};

#[test]
fn test_essence() {
    // Mortals should not have essence
    let mut event_source = CharacterEventSource::default();
    let character_view = event_source.as_character().unwrap();
    assert!(character_view.essence().is_none());

    // Exalts (including Solars) should have essence
    let new_solar = Solar::builder()
        .eclipse()
        .caste_ability(EclipseAbility::Larceny)
        .caste_ability(EclipseAbility::Linguistics)
        .caste_ability(EclipseAbility::Sail)
        .caste_ability(EclipseAbility::Socialize)
        .supernal_ability(EclipseAbility::Presence)
        .favored_ability(AbilityName::Archery)
        .favored_ability(AbilityName::Dodge)
        .favored_ability(AbilityName::Investigation)
        .favored_ability(AbilityName::Performance)
        .favored_ability(AbilityName::Ride)
        .limit_trigger("Being underground or unable to move freely".to_owned())
        .build()
        .unwrap();
    let mutation = CharacterMutation::SetSolar(new_solar);
    event_source.apply_mutation(mutation).unwrap();
    let character_view = event_source.as_character().unwrap();
    assert!(character_view.essence().is_some());
    assert_eq!(character_view.essence().unwrap().rating(), 1);
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 33);
    assert_eq!(mote_state.peripheral().spent(), 0);
    assert_eq!(mote_state.personal().available(), 13);
    assert_eq!(mote_state.personal().spent(), 0);
    assert!(mote_state.committed().next().is_none());

    // Exalts should be able to spend from either peripheral or personal
    let mutation = CharacterMutation::SpendMotes(MotePoolName::Peripheral, 10);
    let character = event_source.apply_mutation(mutation).unwrap();
    let mote_state = character.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 23);
    assert_eq!(mote_state.peripheral().spent(), 10);

    let mutation = CharacterMutation::SpendMotes(MotePoolName::Personal, 10);
    let character = event_source.apply_mutation(mutation).unwrap();
    let mote_state = character.essence().unwrap().motes();
    assert_eq!(mote_state.personal().available(), 3);
    assert_eq!(mote_state.personal().spent(), 10);

    // Exalts should be able to commit from either peripheral or personal, with
    // overflow splitting across pools
    let mutation = CharacterMutation::CommitMotes(
        "Peripheral motes committed".to_owned(),
        MotePoolName::Peripheral,
        10,
    );
    let character = event_source.apply_mutation(mutation).unwrap();
    let mote_state = character.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 13);
    assert_eq!(mote_state.peripheral().spent(), 10);
    assert_eq!(mote_state.personal().available(), 3);
    assert_eq!(mote_state.personal().spent(), 10);

    let mutation = CharacterMutation::CommitMotes(
        "Personal motes committed".to_owned(),
        MotePoolName::Personal,
        10,
    );
    let character = event_source.apply_mutation(mutation).unwrap();
    let mote_state = character.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 6);
    assert_eq!(mote_state.peripheral().spent(), 10);
    assert_eq!(mote_state.personal().available(), 0);
    assert_eq!(mote_state.personal().spent(), 10);
    let mut commits_count = 0;
    for (id, commitment) in mote_state.committed() {
        let peripheral_committed = commitment.peripheral();
        let personal_committed = commitment.personal();
        commits_count += 1;
        if id == MoteCommitmentId::Other("Peripheral motes committed") {
            assert_eq!(peripheral_committed, 10);
            assert_eq!(personal_committed, 0);
        } else if id == MoteCommitmentId::Other("Personal motes committed") {
            assert_eq!(peripheral_committed, 7);
            assert_eq!(personal_committed, 3);
        } else {
            panic!("Unexpected mote commitment");
        }
    }
    assert_eq!(commits_count, 2);

    // Exalts should not be able to spend more motes than they have available
    let mutation = CharacterMutation::SpendMotes(MotePoolName::Peripheral, 255);
    assert!(event_source.apply_mutation(mutation).is_err());
    let mutation = CharacterMutation::SpendMotes(MotePoolName::Personal, 255);
    assert!(event_source.apply_mutation(mutation).is_err());

    // Exalts should not be able to commit more motes than they have available
    let mutation =
        CharacterMutation::CommitMotes("Invalid commit".to_owned(), MotePoolName::Peripheral, 255);
    assert!(event_source.apply_mutation(mutation).is_err());
    let mutation =
        CharacterMutation::CommitMotes("Invalid commit".to_owned(), MotePoolName::Personal, 255);
    assert!(event_source.apply_mutation(mutation).is_err());

    // Recovering essence should refill peripheral first
    let mutation = CharacterMutation::RecoverMotes(10);
    let character = event_source.apply_mutation(mutation).unwrap();
    let mote_state = character.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 16);
    assert_eq!(mote_state.peripheral().spent(), 0);
    assert_eq!(mote_state.personal().available(), 0);
    assert_eq!(mote_state.personal().spent(), 10);

    // ...and personal second
    let mutation = CharacterMutation::RecoverMotes(10);
    let character = event_source.apply_mutation(mutation).unwrap();
    let mote_state = character.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 16);
    assert_eq!(mote_state.peripheral().spent(), 0);
    assert_eq!(mote_state.personal().available(), 10);
    assert_eq!(mote_state.personal().spent(), 0);

    // Uncommitting mote effects should make them spent again
    let mutation = CharacterMutation::UncommitMotes(UncommitMotes::Other(
        "Personal motes committed".to_owned(),
    ));
    let character = event_source.apply_mutation(mutation).unwrap();
    let mote_state = character.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 16);
    assert_eq!(mote_state.peripheral().spent(), 7);
    assert_eq!(mote_state.personal().available(), 10);
    assert_eq!(mote_state.personal().spent(), 3);
    let mut commits_count = 0;
    for (id, commitment) in mote_state.committed() {
        let peripheral_committed = commitment.peripheral();
        let personal_committed = commitment.personal();
        commits_count += 1;
        if id == MoteCommitmentId::Other("Peripheral motes committed") {
            assert_eq!(peripheral_committed, 10);
            assert_eq!(personal_committed, 0);
        } else {
            panic!("Unexpected mote commitment");
        }
    }
    assert_eq!(commits_count, 1);

    let mutation = CharacterMutation::UncommitMotes(UncommitMotes::Other(
        "Peripheral motes committed".to_owned(),
    ));
    let character = event_source.apply_mutation(mutation).unwrap();
    let mote_state = character.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 16);
    assert_eq!(mote_state.peripheral().spent(), 17);
    assert_eq!(mote_state.personal().available(), 10);
    assert_eq!(mote_state.personal().spent(), 3);
    assert!(mote_state.committed().next().is_none());

    // Changing or lowering essence rating should end all mote commitments
    // and refill essence to full
    let mutation =
        CharacterMutation::CommitMotes("Commitment to clear".to_owned(), MotePoolName::Personal, 1);
    event_source.apply_mutation(mutation).unwrap();

    let mutation = CharacterMutation::SetEssenceRating(2);
    let character = event_source.apply_mutation(mutation).unwrap();
    assert_eq!(character.essence().unwrap().rating(), 2);
    let mote_state = character.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 40);
    assert_eq!(mote_state.peripheral().spent(), 0);
    assert_eq!(mote_state.personal().available(), 16);
    assert_eq!(mote_state.personal().spent(), 0);
    assert!(mote_state.committed().next().is_none());

    // Check we can undo full history
    assert!(event_source.undo().is_ok());
    assert!(event_source.undo().is_ok());
    assert!(event_source.undo().is_ok());
    assert!(event_source.undo().is_ok());
    assert!(event_source.undo().is_ok());
    assert!(event_source.undo().is_ok());
    assert!(event_source.undo().is_ok());
    assert!(event_source.undo().is_ok());
    assert!(event_source.undo().is_ok());
    assert!(event_source.undo().is_ok());
    assert!(event_source.undo().is_ok());
    assert!(!event_source.can_undo());

    // Check we can redo full history
    assert!(event_source.redo().is_ok());
    assert!(event_source.redo().is_ok());
    assert!(event_source.redo().is_ok());
    assert!(event_source.redo().is_ok());
    assert!(event_source.redo().is_ok());
    assert!(event_source.redo().is_ok());
    assert!(event_source.redo().is_ok());
    assert!(event_source.redo().is_ok());
    assert!(event_source.redo().is_ok());
    assert!(event_source.redo().is_ok());
    assert!(event_source.redo().is_ok());
    assert!(!event_source.can_redo());

    let character = event_source.as_character().unwrap();
    assert_eq!(character.essence().unwrap().rating(), 2);
    let mote_state = character.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 40);
    assert_eq!(mote_state.peripheral().spent(), 0);
    assert_eq!(mote_state.personal().available(), 16);
    assert_eq!(mote_state.personal().spent(), 0);
    assert!(mote_state.committed().next().is_none());
}
