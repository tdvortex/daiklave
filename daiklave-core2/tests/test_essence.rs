use daiklave_core2::{
    id::Id, Character, CharacterEventSource, CharacterMutation, CharacterView, CommittedMotesId,
    MotePool, SolarTraits,
};

#[test]
fn test_essence_character() {
    // Mortals should not have essence
    let mut character = Character::default();
    assert!(character.essence().is_none());

    // Exalts (including Solars) should have essence
    let solar_traits = SolarTraits::builder().build();
    character.set_solar(&solar_traits).unwrap();
    assert!(character.essence().is_some());
    assert_eq!(character.essence().unwrap().rating(), 1);
    let mote_state = character.essence().unwrap().motes();

    assert_eq!(mote_state.peripheral().available(), 33);
    assert_eq!(mote_state.peripheral().spent(), 0);
    assert_eq!(mote_state.personal().available(), 13);
    assert_eq!(mote_state.personal().spent(), 0);
    assert!(mote_state.committed().next().is_none());

    // Exalts should be able to spend from either peripheral or personal
    assert!(character
        .check_spend_motes(MotePool::Peripheral, 10)
        .is_ok());
    assert!(character.spend_motes(MotePool::Peripheral, 10).is_ok());
    let mote_state = character.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 23);
    assert_eq!(mote_state.peripheral().spent(), 10);
    assert!(character.check_spend_motes(MotePool::Personal, 10).is_ok());
    assert!(character.spend_motes(MotePool::Personal, 10).is_ok());
    let mote_state = character.essence().unwrap().motes();
    assert_eq!(mote_state.personal().available(), 3);
    assert_eq!(mote_state.personal().spent(), 10);

    // Exalts should be able to commit from either peripheral or personal, with
    // overflow splitting across pools
    assert!(character
        .check_commit_motes(
            &CommittedMotesId(Id::Placeholder(1)),
            "Peripheral motes committed",
            MotePool::Peripheral,
            10
        )
        .is_ok());
    assert!(character
        .commit_motes(
            &CommittedMotesId(Id::Placeholder(1)),
            "Peripheral motes committed",
            MotePool::Peripheral,
            10
        )
        .is_ok());
    let mote_state = character.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 13);
    assert_eq!(mote_state.peripheral().spent(), 10);
    assert_eq!(mote_state.personal().available(), 3);
    assert_eq!(mote_state.personal().spent(), 10);
    assert!(character
        .check_commit_motes(
            &CommittedMotesId(Id::Placeholder(2)),
            "Personal motes committed",
            MotePool::Personal,
            10
        )
        .is_ok());
    assert!(character
        .commit_motes(
            &CommittedMotesId(Id::Placeholder(2)),
            "Personal motes committed",
            MotePool::Personal,
            10
        )
        .is_ok());
    let mote_state = character.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 6);
    assert_eq!(mote_state.peripheral().spent(), 10);
    assert_eq!(mote_state.personal().available(), 0);
    assert_eq!(mote_state.personal().spent(), 10);
    let mut commits_count = 0;
    for (id, name, peripheral_committed, personal_committed) in mote_state.committed() {
        commits_count += 1;
        if id == CommittedMotesId(Id::Placeholder(1)) {
            assert_eq!(name, "Peripheral motes committed");
            assert_eq!(peripheral_committed, 10);
            assert_eq!(personal_committed, 0);
        } else if id == CommittedMotesId(Id::Placeholder(2)) {
            assert_eq!(name, "Personal motes committed");
            assert_eq!(peripheral_committed, 7);
            assert_eq!(personal_committed, 3);
        } else {
            panic!("Unexpected mote commitment");
        }
    }
    assert_eq!(commits_count, 2);

    // Exalts should not be able to spend more motes than they have available
    assert!(character
        .check_spend_motes(MotePool::Peripheral, 255)
        .is_err());
    assert!(character
        .check_spend_motes(MotePool::Personal, 255)
        .is_err());

    // Exalts should not be able to commit more motes than they have available
    assert!(character
        .check_commit_motes(
            &CommittedMotesId(Id::Placeholder(3)),
            "Invalid commit",
            MotePool::Peripheral,
            255
        )
        .is_err());
    assert!(character
        .check_commit_motes(
            &CommittedMotesId(Id::Placeholder(3)),
            "Invalid commit",
            MotePool::Personal,
            255
        )
        .is_err());

    // Recovering essence should refill peripheral first
    assert!(character.check_recover_motes(10).is_ok());
    assert!(character.recover_motes(10).is_ok());
    let mote_state = character.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 16);
    assert_eq!(mote_state.peripheral().spent(), 0);
    assert_eq!(mote_state.personal().available(), 0);
    assert_eq!(mote_state.personal().spent(), 10);

    // ...and personal second
    assert!(character.check_recover_motes(10).is_ok());
    assert!(character.recover_motes(10).is_ok());
    let mote_state = character.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 16);
    assert_eq!(mote_state.peripheral().spent(), 0);
    assert_eq!(mote_state.personal().available(), 10);
    assert_eq!(mote_state.personal().spent(), 0);

    // Uncommitting mote effects should make them spent again
    assert!(character
        .check_uncommit_motes(&CommittedMotesId(Id::Placeholder(2)))
        .is_ok());
    assert!(character
        .uncommit_motes(&CommittedMotesId(Id::Placeholder(2)))
        .is_ok());
    let mote_state = character.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 16);
    assert_eq!(mote_state.peripheral().spent(), 7);
    assert_eq!(mote_state.personal().available(), 10);
    assert_eq!(mote_state.personal().spent(), 3);
    let mut commits_count = 0;
    for (id, name, peripheral_committed, personal_committed) in mote_state.committed() {
        commits_count += 1;
        if id == CommittedMotesId(Id::Placeholder(1)) {
            assert_eq!(name, "Peripheral motes committed");
            assert_eq!(peripheral_committed, 10);
            assert_eq!(personal_committed, 0);
        } else {
            panic!("Unexpected mote commitment");
        }
    }
    assert_eq!(commits_count, 1);

    assert!(character
        .check_uncommit_motes(&CommittedMotesId(Id::Placeholder(1)))
        .is_ok());
    assert!(character
        .uncommit_motes(&CommittedMotesId(Id::Placeholder(1)))
        .is_ok());
    let mote_state = character.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 16);
    assert_eq!(mote_state.peripheral().spent(), 17);
    assert_eq!(mote_state.personal().available(), 10);
    assert_eq!(mote_state.personal().spent(), 3);
    assert!(mote_state.committed().next().is_none());

    // Changing or lowering essence rating should end all mote commitments
    // and refill essence to full
    assert!(character
        .commit_motes(
            &CommittedMotesId(Id::Placeholder(3)),
            "Commitment to clear",
            MotePool::Personal,
            1
        )
        .is_ok());
    assert!(character.check_set_essence_rating(2).is_ok());
    assert!(character.set_essence_rating(2).is_ok());
    assert_eq!(character.essence().unwrap().rating(), 2);
    let mote_state = character.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 40);
    assert_eq!(mote_state.peripheral().spent(), 0);
    assert_eq!(mote_state.personal().available(), 16);
    assert_eq!(mote_state.personal().spent(), 0);
    assert!(mote_state.committed().next().is_none());
}

#[test]
fn test_essence_character_view() {
    // Mortals should not have essence
    let mut character_view = CharacterView::default();
    assert!(character_view.essence().is_none());

    // Exalts (including Solars) should have essence
    let solar_traits = SolarTraits::builder().build();
    character_view.set_solar(&solar_traits).unwrap();
    assert!(character_view.essence().is_some());
    assert_eq!(character_view.essence().unwrap().rating(), 1);
    let mote_state = character_view.essence().unwrap().motes();

    assert_eq!(mote_state.peripheral().available(), 33);
    assert_eq!(mote_state.peripheral().spent(), 0);
    assert_eq!(mote_state.personal().available(), 13);
    assert_eq!(mote_state.personal().spent(), 0);
    assert!(mote_state.committed().next().is_none());

    // Exalts should be able to spend from either peripheral or personal
    assert!(character_view
        .check_spend_motes(MotePool::Peripheral, 10)
        .is_ok());
    assert!(character_view.spend_motes(MotePool::Peripheral, 10).is_ok());
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 23);
    assert_eq!(mote_state.peripheral().spent(), 10);
    assert!(character_view
        .check_spend_motes(MotePool::Personal, 10)
        .is_ok());
    assert!(character_view.spend_motes(MotePool::Personal, 10).is_ok());
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.personal().available(), 3);
    assert_eq!(mote_state.personal().spent(), 10);

    // Exalts should be able to commit from either peripheral or personal, with
    // overflow splitting across pools
    assert!(character_view
        .check_commit_motes(
            &CommittedMotesId(Id::Placeholder(1)),
            "Peripheral motes committed",
            MotePool::Peripheral,
            10
        )
        .is_ok());
    assert!(character_view
        .commit_motes(
            &CommittedMotesId(Id::Placeholder(1)),
            "Peripheral motes committed",
            MotePool::Peripheral,
            10
        )
        .is_ok());
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 13);
    assert_eq!(mote_state.peripheral().spent(), 10);
    assert_eq!(mote_state.personal().available(), 3);
    assert_eq!(mote_state.personal().spent(), 10);
    assert!(character_view
        .check_commit_motes(
            &CommittedMotesId(Id::Placeholder(2)),
            "Personal motes committed",
            MotePool::Personal,
            10
        )
        .is_ok());
    assert!(character_view
        .commit_motes(
            &CommittedMotesId(Id::Placeholder(2)),
            "Personal motes committed",
            MotePool::Personal,
            10
        )
        .is_ok());
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 6);
    assert_eq!(mote_state.peripheral().spent(), 10);
    assert_eq!(mote_state.personal().available(), 0);
    assert_eq!(mote_state.personal().spent(), 10);
    let mut commits_count = 0;
    for (id, name, peripheral_committed, personal_committed) in mote_state.committed() {
        commits_count += 1;
        if id == CommittedMotesId(Id::Placeholder(1)) {
            assert_eq!(name, "Peripheral motes committed");
            assert_eq!(peripheral_committed, 10);
            assert_eq!(personal_committed, 0);
        } else if id == CommittedMotesId(Id::Placeholder(2)) {
            assert_eq!(name, "Personal motes committed");
            assert_eq!(peripheral_committed, 7);
            assert_eq!(personal_committed, 3);
        } else {
            panic!("Unexpected mote commitment");
        }
    }
    assert_eq!(commits_count, 2);

    // Exalts should not be able to spend more motes than they have available
    assert!(character_view
        .check_spend_motes(MotePool::Peripheral, 255)
        .is_err());
    assert!(character_view
        .check_spend_motes(MotePool::Personal, 255)
        .is_err());

    // Exalts should not be able to commit more motes than they have available
    assert!(character_view
        .check_commit_motes(
            &CommittedMotesId(Id::Placeholder(3)),
            "Invalid commit",
            MotePool::Peripheral,
            255
        )
        .is_err());
    assert!(character_view
        .check_commit_motes(
            &CommittedMotesId(Id::Placeholder(3)),
            "Invalid commit",
            MotePool::Personal,
            255
        )
        .is_err());

    // Recovering essence should refill peripheral first
    assert!(character_view.check_recover_motes(10).is_ok());
    assert!(character_view.recover_motes(10).is_ok());
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 16);
    assert_eq!(mote_state.peripheral().spent(), 0);
    assert_eq!(mote_state.personal().available(), 0);
    assert_eq!(mote_state.personal().spent(), 10);

    // ...and personal second
    assert!(character_view.check_recover_motes(10).is_ok());
    assert!(character_view.recover_motes(10).is_ok());
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 16);
    assert_eq!(mote_state.peripheral().spent(), 0);
    assert_eq!(mote_state.personal().available(), 10);
    assert_eq!(mote_state.personal().spent(), 0);

    // Uncommitting mote effects should make them spent again
    assert!(character_view
        .check_uncommit_motes(&CommittedMotesId(Id::Placeholder(2)))
        .is_ok());
    assert!(character_view
        .uncommit_motes(&CommittedMotesId(Id::Placeholder(2)))
        .is_ok());
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 16);
    assert_eq!(mote_state.peripheral().spent(), 7);
    assert_eq!(mote_state.personal().available(), 10);
    assert_eq!(mote_state.personal().spent(), 3);
    let mut commits_count = 0;
    for (id, name, peripheral_committed, personal_committed) in mote_state.committed() {
        commits_count += 1;
        if id == CommittedMotesId(Id::Placeholder(1)) {
            assert_eq!(name, "Peripheral motes committed");
            assert_eq!(peripheral_committed, 10);
            assert_eq!(personal_committed, 0);
        } else {
            panic!("Unexpected mote commitment");
        }
    }
    assert_eq!(commits_count, 1);

    assert!(character_view
        .check_uncommit_motes(&CommittedMotesId(Id::Placeholder(1)))
        .is_ok());
    assert!(character_view
        .uncommit_motes(&CommittedMotesId(Id::Placeholder(1)))
        .is_ok());
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 16);
    assert_eq!(mote_state.peripheral().spent(), 17);
    assert_eq!(mote_state.personal().available(), 10);
    assert_eq!(mote_state.personal().spent(), 3);
    assert!(mote_state.committed().next().is_none());

    // Changing or lowering essence rating should end all mote commitments
    // and refill essence to full
    assert!(character_view
        .commit_motes(
            &CommittedMotesId(Id::Placeholder(3)),
            "Commitment to clear",
            MotePool::Personal,
            1
        )
        .is_ok());
    assert!(character_view.check_set_essence_rating(2).is_ok());
    assert!(character_view.set_essence_rating(2).is_ok());
    assert_eq!(character_view.essence().unwrap().rating(), 2);
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 40);
    assert_eq!(mote_state.peripheral().spent(), 0);
    assert_eq!(mote_state.personal().available(), 16);
    assert_eq!(mote_state.personal().spent(), 0);
    assert!(mote_state.committed().next().is_none());
}

#[test]
fn test_essence_character_event_source() {
    // Mortals should not have essence
    let mut event_source = CharacterEventSource::default();
    let character_view = event_source.as_character_view().unwrap();
    assert!(character_view.essence().is_none());

    // Exalts (including Solars) should have essence
    let solar_traits = SolarTraits::builder().build();
    let mutation = CharacterMutation::SetSolar(solar_traits);
    event_source.apply_mutation(mutation).unwrap();
    let character_view = event_source.as_character_view().unwrap();
    assert!(character_view.essence().is_some());
    assert_eq!(character_view.essence().unwrap().rating(), 1);
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 33);
    assert_eq!(mote_state.peripheral().spent(), 0);
    assert_eq!(mote_state.personal().available(), 13);
    assert_eq!(mote_state.personal().spent(), 0);
    assert!(mote_state.committed().next().is_none());

    // Exalts should be able to spend from either peripheral or personal
    let mutation = CharacterMutation::SpendMotes(MotePool::Peripheral, 10);
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 23);
    assert_eq!(mote_state.peripheral().spent(), 10);

    let mutation = CharacterMutation::SpendMotes(MotePool::Personal, 10);
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.personal().available(), 3);
    assert_eq!(mote_state.personal().spent(), 10);

    // Exalts should be able to commit from either peripheral or personal, with
    // overflow splitting across pools
    let mutation = CharacterMutation::CommitMotes(
        CommittedMotesId(Id::Placeholder(1)),
        "Peripheral motes committed".to_owned(),
        MotePool::Peripheral,
        10,
    );
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 13);
    assert_eq!(mote_state.peripheral().spent(), 10);
    assert_eq!(mote_state.personal().available(), 3);
    assert_eq!(mote_state.personal().spent(), 10);

    let mutation = CharacterMutation::CommitMotes(
        CommittedMotesId(Id::Placeholder(2)),
        "Personal motes committed".to_owned(),
        MotePool::Personal,
        10,
    );
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 6);
    assert_eq!(mote_state.peripheral().spent(), 10);
    assert_eq!(mote_state.personal().available(), 0);
    assert_eq!(mote_state.personal().spent(), 10);
    let mut commits_count = 0;
    for (id, name, peripheral_committed, personal_committed) in mote_state.committed() {
        commits_count += 1;
        if id == CommittedMotesId(Id::Placeholder(1)) {
            assert_eq!(name, "Peripheral motes committed");
            assert_eq!(peripheral_committed, 10);
            assert_eq!(personal_committed, 0);
        } else if id == CommittedMotesId(Id::Placeholder(2)) {
            assert_eq!(name, "Personal motes committed");
            assert_eq!(peripheral_committed, 7);
            assert_eq!(personal_committed, 3);
        } else {
            panic!("Unexpected mote commitment");
        }
    }
    assert_eq!(commits_count, 2);

    // Exalts should not be able to spend more motes than they have available
    let mutation = CharacterMutation::SpendMotes(MotePool::Peripheral, 255);
    assert!(character_view.check_mutation(&mutation).is_err());
    let mutation = CharacterMutation::SpendMotes(MotePool::Personal, 255);
    assert!(character_view.check_mutation(&mutation).is_err());

    // Exalts should not be able to commit more motes than they have available
    let mutation = CharacterMutation::CommitMotes(
        CommittedMotesId(Id::Placeholder(3)),
        "Invalid commit".to_owned(),
        MotePool::Peripheral,
        255,
    );
    assert!(character_view.check_mutation(&mutation).is_err());
    let mutation = CharacterMutation::CommitMotes(
        CommittedMotesId(Id::Placeholder(3)),
        "Invalid commit".to_owned(),
        MotePool::Personal,
        255,
    );
    assert!(character_view.check_mutation(&mutation).is_err());

    // Recovering essence should refill peripheral first
    let mutation = CharacterMutation::RecoverMotes(10);
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 16);
    assert_eq!(mote_state.peripheral().spent(), 0);
    assert_eq!(mote_state.personal().available(), 0);
    assert_eq!(mote_state.personal().spent(), 10);

    // ...and personal second
    let mutation = CharacterMutation::RecoverMotes(10);
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 16);
    assert_eq!(mote_state.peripheral().spent(), 0);
    assert_eq!(mote_state.personal().available(), 10);
    assert_eq!(mote_state.personal().spent(), 0);

    // Uncommitting mote effects should make them spent again
    let mutation = CharacterMutation::UncommitMotes(CommittedMotesId(Id::Placeholder(2)));
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 16);
    assert_eq!(mote_state.peripheral().spent(), 7);
    assert_eq!(mote_state.personal().available(), 10);
    assert_eq!(mote_state.personal().spent(), 3);
    let mut commits_count = 0;
    for (id, name, peripheral_committed, personal_committed) in mote_state.committed() {
        commits_count += 1;
        if id == CommittedMotesId(Id::Placeholder(1)) {
            assert_eq!(name, "Peripheral motes committed");
            assert_eq!(peripheral_committed, 10);
            assert_eq!(personal_committed, 0);
        } else {
            panic!("Unexpected mote commitment");
        }
    }
    assert_eq!(commits_count, 1);

    let mutation = CharacterMutation::UncommitMotes(CommittedMotesId(Id::Placeholder(1)));
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 16);
    assert_eq!(mote_state.peripheral().spent(), 17);
    assert_eq!(mote_state.personal().available(), 10);
    assert_eq!(mote_state.personal().spent(), 3);
    assert!(mote_state.committed().next().is_none());

    // Changing or lowering essence rating should end all mote commitments
    // and refill essence to full
    let mutation = CharacterMutation::CommitMotes(
        CommittedMotesId(Id::Placeholder(3)),
        "Commitment to clear".to_owned(),
        MotePool::Personal,
        1,
    );
    event_source.apply_mutation(mutation).unwrap();
    let character_view = event_source.as_character_view().unwrap();
    let mutation = CharacterMutation::SetEssenceRating(2);
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.essence().unwrap().rating(), 2);
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 40);
    assert_eq!(mote_state.peripheral().spent(), 0);
    assert_eq!(mote_state.personal().available(), 16);
    assert_eq!(mote_state.personal().spent(), 0);
    assert!(mote_state.committed().next().is_none());

    // Check we can undo full history
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(!event_source.can_undo());

    // Check we can redo full history
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(!event_source.can_redo());

    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(character_view.essence().unwrap().rating(), 2);
    let mote_state = character_view.essence().unwrap().motes();
    assert_eq!(mote_state.peripheral().available(), 40);
    assert_eq!(mote_state.peripheral().spent(), 0);
    assert_eq!(mote_state.personal().available(), 16);
    assert_eq!(mote_state.personal().spent(), 0);
    assert!(mote_state.committed().next().is_none());
}