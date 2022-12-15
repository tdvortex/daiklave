use std::collections::HashSet;

use exalted_3e_gui::{
    character::CharacterBuilder,
    id::Id,
    intimacies::{Intimacy, IntimacyLevel, IntimacyType},
};

pub fn create_initial_intimacies(builder: CharacterBuilder) -> CharacterBuilder {
    builder
        .with_intimacy(Intimacy::new(
            IntimacyLevel::Defining,
            IntimacyType::Principle,
            "Never stand idle against injustice".to_owned(),
            Id::Placeholder(0),
        ))
        .with_intimacy(Intimacy::new(
            IntimacyLevel::Major,
            IntimacyType::Tie,
            "Ragara Tirnis (Love)".to_owned(),
            Id::Placeholder(1),
        ))
        .with_intimacy(Intimacy::new(
            IntimacyLevel::Major,
            IntimacyType::Tie,
            "Mask of Winters (Revenge)".to_owned(),
            Id::Placeholder(2),
        ))
        .with_intimacy(Intimacy::new(
            IntimacyLevel::Minor,
            IntimacyType::Tie,
            "Street Vendors (Camaraderie)".to_owned(),
            Id::Placeholder(3),
        ))
}

pub fn validate_initial_intimacies(intimacies: &Vec<Intimacy>, should_have_id: bool) {
    let expected: HashSet<(IntimacyLevel, IntimacyType, &str)> = [
        (
            IntimacyLevel::Defining,
            IntimacyType::Principle,
            "Never stand idle against injustice",
        ),
        (
            IntimacyLevel::Major,
            IntimacyType::Tie,
            "Ragara Tirnis (Love)",
        ),
        (
            IntimacyLevel::Major,
            IntimacyType::Tie,
            "Mask of Winters (Revenge)",
        ),
        (
            IntimacyLevel::Minor,
            IntimacyType::Tie,
            "Street Vendors (Camaraderie)",
        ),
    ]
    .into();

    assert_eq!(expected.len(), intimacies.len());

    for actual in intimacies.iter() {
        assert_eq!(!actual.id().is_placeholder(), should_have_id);
        assert!(expected.contains(&(
            actual.intimacy_level,
            actual.intimacy_type,
            actual.description.as_str()
        )));
    }
}

pub fn modify_intimacies(intimacies: &mut Vec<Intimacy>) {
    let mut remove_index = None;

    for (index, i_ptr) in intimacies.iter_mut().enumerate() {
        if i_ptr.description.as_str() == "Street Vendors (Camaraderie)" {
            remove_index = Some(index);
        } else if i_ptr.description.as_str() == "Ragara Tirnis (Love)" {
            // Modify an intimacy
            i_ptr.description = "Ragara Tirnis (Broken Heart)".to_owned();
            i_ptr.intimacy_level = IntimacyLevel::Defining;
        }
    }

    // Remove an intimacy
    intimacies.remove(remove_index.unwrap());

    // Add an intimacy
    intimacies.push(Intimacy::new(
        IntimacyLevel::Minor,
        IntimacyType::Principle,
        "Trust leads pain".to_owned(),
        Id::Placeholder(4),
    ));
}

pub fn validate_modified_intimacies(intimacies: &Vec<Intimacy>) {
    let expected: HashSet<(IntimacyLevel, IntimacyType, &str)> = [
        (
            IntimacyLevel::Defining,
            IntimacyType::Principle,
            "Never stand idle against injustice",
        ),
        (
            IntimacyLevel::Defining,
            IntimacyType::Tie,
            "Ragara Tirnis (Broken Heart)",
        ),
        (
            IntimacyLevel::Major,
            IntimacyType::Tie,
            "Mask of Winters (Revenge)",
        ),
        (
            IntimacyLevel::Minor,
            IntimacyType::Principle,
            "Trust leads pain",
        ),
    ]
    .into();

    assert_eq!(expected.len(), intimacies.len());

    for actual in intimacies.iter() {
        assert!(expected.contains(&(
            actual.intimacy_level,
            actual.intimacy_type,
            actual.description.as_str()
        )));
    }
}
