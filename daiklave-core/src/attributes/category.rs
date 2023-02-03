use super::AttributeName;

/// The category of an attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeCategory {
    /// Strength, Dexterity, and Stamina
    Physical,
    /// Perception, Intelligence, and Wits
    Mental,
    /// Charisma, Manipulation, Appearance
    Social,
}

impl From<AttributeName> for AttributeCategory {
    fn from(name: AttributeName) -> Self {
        match name {
            AttributeName::Strength => Self::Physical,
            AttributeName::Dexterity => Self::Physical,
            AttributeName::Stamina => Self::Physical,
            AttributeName::Charisma => Self::Social,
            AttributeName::Manipulation => Self::Social,
            AttributeName::Appearance => Self::Social,
            AttributeName::Perception => Self::Mental,
            AttributeName::Intelligence => Self::Mental,
            AttributeName::Wits => Self::Mental,
        }
    }
}
