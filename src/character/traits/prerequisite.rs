use super::{abilities::{AbilityNameNoFocus}, attributes::AttributeName};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Prerequisite {
    Ability(AbilityPrerequisite),
    Attribute(AttributePrerequisite),
    Essence(u8),
    Charm(i32),
    ExaltType(ExaltTypePrerequisite),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AbilityPrerequisite {
    pub ability_name: AbilityNameNoFocus,
    pub subskill: Option<String>,
    pub dots: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct AttributePrerequisite {
    pub attribute_name: AttributeName,
    pub dots: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExaltTypePrerequisite {
    Solar,
    Lunar,
    DragonBlooded,
    Spirit,
    SpiritOrEclipse,
}

pub type PrerequisiteSet = Vec<Prerequisite>;
