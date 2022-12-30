use daiklave_core2::{id::Id, AbilityName, Character, CommittedMotesId, Eclipse, MotePool, Solar};

#[test]
fn test_essence_character() {
    // Mortals should not have essence
    let mut character = Character::default();
    assert!(character.essence().is_none());

    // Exalts (including Solars) should have essence
    let eclipse = {
        let mut builder = Eclipse::builder();
        [
            AbilityName::Larceny,
            AbilityName::Linguistics,
            AbilityName::Sail,
            AbilityName::Socialize,
        ]
        .into_iter()
        .for_each(|ability| {
            builder.add_caste_ability(ability).unwrap();
        });
        builder.set_supernal_ability(AbilityName::Presence).unwrap();
        builder.build().unwrap()
    };

    let solar_traits = {
        let mut builder = Solar::builder();
        builder.set_eclipse(eclipse);
        [
            AbilityName::Archery,
            AbilityName::Dodge,
            AbilityName::Investigation,
            AbilityName::Performance,
            AbilityName::Ride,
        ]
        .into_iter()
        .for_each(|ability| {
            builder.add_favored_ability(ability).unwrap();
        });
        builder.build().unwrap()
    };

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
