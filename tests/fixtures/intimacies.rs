use exalted_3e_gui::{character::CharacterBuilder, intimacies::{Intimacy, IntimacyLevel, IntimacyType}};

pub fn create_initial_intimacites(builder: CharacterBuilder) -> CharacterBuilder {
    builder.with_intimacy(Intimacy::new(
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