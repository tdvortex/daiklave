use crate::character::traits::attributes::AttributeName;

pub enum ExaltType {
    Solar,
    Lunar,
    DragonBlooded,
}

impl TryFrom<String> for ExaltType {
    type Error = eyre::Report;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "SO" => Ok(Self::Solar),
            "LU" => Ok(Self::Lunar),
            "DB" => Ok(Self::DragonBlooded),
            _ => Err(eyre::eyre!("unknown exalt type encoding: {}", value))
        }
    }
}

impl Into<String> for ExaltType {
    fn into(self) -> String {
        match self {
            ExaltType::Solar => "SO".to_owned(),
            ExaltType::Lunar => "LU".to_owned(),
            ExaltType::DragonBlooded => "DB".to_owned(),
        }
    }
}

impl TryFrom<String> for AttributeName {
    type Error = eyre::Report;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "STR" => Ok(Self::Strength),
            "DEX" => Ok(Self::Dexterity),
            "STA" => Ok(Self::Stamina),
            "CHA" => Ok(Self::Charisma),
            "MAN" => Ok(Self::Manipulation),
            "APP" => Ok(Self::Appearance),
            "PER" => Ok(Self::Perception),
            "INT" => Ok(Self::Intelligence),
            "WIT" => Ok(Self::Wits),
            _ => Err(eyre::eyre!("unknown attribute encoding: {}", value))
        }
    }
}

impl Into<String> for AttributeName {
    fn into(self) -> String {
        match self {
            crate::character::traits::attributes::AttributeName::Strength => "STR".to_owned(),
            crate::character::traits::attributes::AttributeName::Dexterity => "DEX".to_owned(),
            crate::character::traits::attributes::AttributeName::Stamina => "STA".to_owned(),
            crate::character::traits::attributes::AttributeName::Charisma => "CHA".to_owned(),
            crate::character::traits::attributes::AttributeName::Manipulation => "MAN".to_owned(),
            crate::character::traits::attributes::AttributeName::Appearance => "APP".to_owned(),
            crate::character::traits::attributes::AttributeName::Perception => "PER".to_owned(),
            crate::character::traits::attributes::AttributeName::Intelligence => "INT".to_owned(),
            crate::character::traits::attributes::AttributeName::Wits => "WIT".to_owned(),
        }
    }
}