use std::iter::{ExactSizeIterator, FusedIterator};

pub trait HasAttributes {
    fn attributes_iter(&self) -> AttributesIter;
    fn get_attribute(&self, attribute_name: &AttributeName) -> AttributeValue;
    fn set_attribute(&mut self, attribute_name: &AttributeName, new_value: AttributeValue);
}

// Attributes are nonnegative integers
// Usually rated 1 to 5, but may be 6+ in some cases
pub type AttributeValue = u8;

#[derive(Clone, Copy)]
pub enum AttributeName {
    Strength,
    Dexterity,
    Stamina,
    Charisma,
    Manipulation,
    Appearance,
    Perception,
    Intelligence,
    Wits,
}

impl AttributeName {
    fn iter() -> AttributeNameIter {
        AttributeNameIter {
            next_name: Some(AttributeName::Strength),
        }
    }
}

struct AttributeNameIter {
    next_name: Option<AttributeName>,
}

impl Iterator for AttributeNameIter {
    type Item = AttributeName;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match &self.next_name {
            None => None,
            Some(AttributeName::Strength) => Some(AttributeName::Dexterity),
            Some(AttributeName::Dexterity) => Some(AttributeName::Stamina),
            Some(AttributeName::Stamina) => Some(AttributeName::Charisma),
            Some(AttributeName::Charisma) => Some(AttributeName::Manipulation),
            Some(AttributeName::Manipulation) => Some(AttributeName::Appearance),
            Some(AttributeName::Appearance) => Some(AttributeName::Perception),
            Some(AttributeName::Perception) => Some(AttributeName::Intelligence),
            Some(AttributeName::Intelligence) => Some(AttributeName::Wits),
            Some(AttributeName::Wits) => None,
        };
        let out = self.next_name;
        self.next_name = next;
        out
    }
}

impl ExactSizeIterator for AttributeNameIter {
    fn len(&self) -> usize {
        9
    }
}

impl FusedIterator for AttributeNameIter {}

#[derive(Debug)]
pub struct Attributes {
    strength: AttributeValue,
    dexterity: AttributeValue,
    stamina: AttributeValue,
    charisma: AttributeValue,
    manipulation: AttributeValue,
    appearance: AttributeValue,
    perception: AttributeValue,
    intelligence: AttributeValue,
    wits: AttributeValue,
}

// Attributes default to 1, not 0
impl Default for Attributes {
    fn default() -> Self {
        Self {
            strength: 1,
            dexterity: 1,
            stamina: 1,
            charisma: 1,
            manipulation: 1,
            appearance: 1,
            perception: 1,
            intelligence: 1,
            wits: 1,
        }
    }
}

impl Attributes {
    pub fn get(&self, attribute_name: &AttributeName) -> AttributeValue {
        match attribute_name {
            AttributeName::Strength => self.strength,
            AttributeName::Dexterity => self.dexterity,
            AttributeName::Stamina => self.stamina,
            AttributeName::Charisma => self.charisma,
            AttributeName::Manipulation => self.manipulation,
            AttributeName::Appearance => self.appearance,
            AttributeName::Perception => self.perception,
            AttributeName::Intelligence => self.intelligence,
            AttributeName::Wits => self.wits,
        }
    }

    pub fn set(&mut self, attribute_name: &AttributeName, new_value: AttributeValue) {
        match attribute_name {
            AttributeName::Strength => self.strength = new_value,
            AttributeName::Dexterity => self.dexterity = new_value,
            AttributeName::Stamina => self.stamina = new_value,
            AttributeName::Charisma => self.charisma = new_value,
            AttributeName::Manipulation => self.manipulation = new_value,
            AttributeName::Appearance => self.appearance = new_value,
            AttributeName::Perception => self.perception = new_value,
            AttributeName::Intelligence => self.intelligence = new_value,
            AttributeName::Wits => self.wits = new_value,
        }
    }

    pub fn iter(&self) -> AttributesIter {
        AttributesIter {
            attributes: self,
            name_iter: AttributeName::iter(),
        }
    }
}

pub struct AttributesIter<'a> {
    attributes: &'a Attributes,
    name_iter: AttributeNameIter,
}

impl<'a> Iterator for AttributesIter<'a> {
    type Item = (AttributeName, AttributeValue);

    fn next(&mut self) -> Option<Self::Item> {
        let attribute_name = self.name_iter.next()?;

        Some((attribute_name, self.attributes.get(&attribute_name)))
    }
}

impl<'a> ExactSizeIterator for AttributesIter<'a> {
    fn len(&self) -> usize {
        9
    }
}

impl<'a> FusedIterator for AttributesIter<'a> {}
