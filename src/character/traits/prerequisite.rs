use super::{abilities::AbilityName, attributes::AttributeName};

pub struct AbilityPrerequisite {
    pub ability_name: AbilityName,
    pub level: u8,
}

pub struct AttributePrerequisite {
    pub attribute_name: AttributeName,
    pub level: u8,
}

pub enum Prerequisite {
    Ability(AbilityPrerequisite),
    Attribute(AttributePrerequisite),
    Essence(u8),
    Charm(String),
}

pub enum ExaltTypePrerequisite {
    Solar,
    Lunar,
    DragonBlooded,
    Spirit,
    SpiritOrEclipse,
}

pub enum PrerequisiteSet {
    Any(Vec<Prerequisite>),
    All(Vec<Prerequisite>),
}
