use exalted_3e_gui::{
    character::CharacterBuilder,
    intimacies::{Intimacy, IntimacyLevel, IntimacyType},
};

pub fn create_initial_intimacies(builder: CharacterBuilder) -> CharacterBuilder {
    builder
        .with_intimacy(Intimacy::new(
            IntimacyLevel::Defining,
            IntimacyType::Principle,
            "Never stand idle against injustice".to_owned(),
            None,
        ))
        .with_intimacy(Intimacy::new(
            IntimacyLevel::Major,
            IntimacyType::Tie,
            "Ragara Tirnis (Love)".to_owned(),
            None,
        ))
        .with_intimacy(Intimacy::new(
            IntimacyLevel::Major,
            IntimacyType::Tie,
            "Mask of Winters (Revenge)".to_owned(),
            None,
        ))
        .with_intimacy(Intimacy::new(
            IntimacyLevel::Minor,
            IntimacyType::Tie,
            "Street Vendors (Camaraderie)".to_owned(),
            None,
        ))
}

pub fn validate_initial_intimacies(intimacies: &Vec<Intimacy>, should_have_id: bool) {
    for (expected, actual) in vec![
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
    .iter()
    .zip(intimacies.iter())
    {
        assert_eq!(actual.id().is_some(), should_have_id);
        assert_eq!(expected.0, actual.intimacy_level);
        assert_eq!(expected.1, actual.intimacy_type);
        assert_eq!(expected.2, actual.description);
    }
}
