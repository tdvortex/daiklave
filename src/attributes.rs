use eyre::{eyre, Result};
use std::iter::{ExactSizeIterator, FusedIterator};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

pub struct Attribute<'a> {
    name: AttributeName,
    value: &'a u8,
}

impl<'a> Attribute<'a> {
    pub fn name(&self) -> AttributeName {
        self.name
    }

    pub fn dots(&self) -> u8 {
        *self.value
    }
}

pub struct AttributeMut<'a> {
    name: AttributeName,
    value: &'a mut u8,
}

impl<'a> AttributeMut<'a> {
    pub fn name(&self) -> AttributeName {
        self.name
    }

    pub fn dots(&self) -> u8 {
        *self.value
    }

    pub fn set_value(&mut self, new_value: u8) -> Result<()> {
        if new_value > 0 {
            *self.value = new_value;
            Ok(())
        } else {
            Err(eyre!("attributes must be 1 or more"))
        }
    }
}

#[derive(Debug)]
pub struct Attributes {
    strength: u8,
    dexterity: u8,
    stamina: u8,
    charisma: u8,
    manipulation: u8,
    appearance: u8,
    perception: u8,
    intelligence: u8,
    wits: u8,
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
    pub fn get(&self, attribute_name: &AttributeName) -> Attribute {
        Attribute {
            name: *attribute_name,
            value: match attribute_name {
                AttributeName::Strength => &self.strength,
                AttributeName::Dexterity => &self.dexterity,
                AttributeName::Stamina => &self.stamina,
                AttributeName::Charisma => &self.charisma,
                AttributeName::Manipulation => &self.manipulation,
                AttributeName::Appearance => &self.appearance,
                AttributeName::Perception => &self.perception,
                AttributeName::Intelligence => &self.intelligence,
                AttributeName::Wits => &self.wits,
            },
        }
    }

    pub fn get_mut(&mut self, attribute_name: &AttributeName) -> AttributeMut {
        AttributeMut {
            name: *attribute_name,
            value: match attribute_name {
                AttributeName::Strength => &mut self.strength,
                AttributeName::Dexterity => &mut self.dexterity,
                AttributeName::Stamina => &mut self.stamina,
                AttributeName::Charisma => &mut self.charisma,
                AttributeName::Manipulation => &mut self.manipulation,
                AttributeName::Appearance => &mut self.appearance,
                AttributeName::Perception => &mut self.perception,
                AttributeName::Intelligence => &mut self.intelligence,
                AttributeName::Wits => &mut self.wits,
            },
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
    type Item = Attribute<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let attribute_name = self.name_iter.next()?;
        Some(self.attributes.get(&attribute_name))
    }
}

impl<'a> ExactSizeIterator for AttributesIter<'a> {
    fn len(&self) -> usize {
        9
    }
}

impl<'a> FusedIterator for AttributesIter<'a> {}
