use crate::{abilities::AbilityName, attributes::AttributeName};

pub type Charms = Vec<Charm>;

pub struct BookReference {
    pub book_name: String,
    pub page_number: usize,
}

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


pub struct Charm {
    pub name: String,
    pub reference: Option<BookReference>,
    pub summary: String,
    pub full_description: String,
}