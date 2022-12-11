use exalted_3e_gui::{character::CharacterBuilder, attributes::AttributeName};

pub fn create_attributes(builder: CharacterBuilder) -> CharacterBuilder {
    vec![
        (AttributeName::Strength, 4),
        (AttributeName::Dexterity, 4),
        (AttributeName::Stamina, 3),
        (AttributeName::Charisma, 4),
        (AttributeName::Manipulation, 3),
        (AttributeName::Appearance, 2),
        (AttributeName::Intelligence, 3),
        (AttributeName::Wits, 3),
    ]
    .into_iter()
    .fold(builder, |ic, (attribute_name, value)| {
        ic.with_attribute(attribute_name, value).unwrap()
    })
}