use std::collections::HashMap;

use exalted_3e_gui::{
    attributes::{AttributeName, Attributes},
    character::CharacterBuilder,
};

pub fn create_initial_attributes(builder: CharacterBuilder) -> CharacterBuilder {
    vec![
        (AttributeName::Strength, 4),
        (AttributeName::Dexterity, 4),
        (AttributeName::Stamina, 3),
        (AttributeName::Charisma, 4),
        (AttributeName::Manipulation, 3),
        (AttributeName::Appearance, 2),
        (AttributeName::Intelligence, 3),
        (AttributeName::Wits, 3),
        (AttributeName::Perception, 1),
    ]
    .into_iter()
    .fold(builder, |ic, (attribute_name, value)| {
        ic.with_attribute(attribute_name, value).unwrap()
    })
}

pub fn validate_initial_attributes(attributes: &Attributes) {
    assert_eq!(
        attributes
            .iter()
            .map(|attr| (attr.name(), attr.dots()))
            .collect::<HashMap<AttributeName, u8>>(),
        vec![
            (AttributeName::Strength, 4),
            (AttributeName::Dexterity, 4),
            (AttributeName::Stamina, 3),
            (AttributeName::Charisma, 4),
            (AttributeName::Manipulation, 3),
            (AttributeName::Appearance, 2),
            (AttributeName::Intelligence, 3),
            (AttributeName::Wits, 3),
            (AttributeName::Perception, 1)
        ]
        .into_iter()
        .collect::<HashMap::<AttributeName, u8>>()
    );
}
