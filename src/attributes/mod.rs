use serde::{Deserialize, Serialize};
pub(crate) mod tables;
pub(crate) mod update;
pub use update::AttributesDiff;

use eyre::{eyre, Result};
use std::iter::{ExactSizeIterator, FusedIterator};

use crate::prerequisite::AttributePrerequisite;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash)]
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
        match self.next_name {
            None => 0,
            Some(AttributeName::Strength) => 9,
            Some(AttributeName::Dexterity) => 8,
            Some(AttributeName::Stamina) => 7,
            Some(AttributeName::Charisma) => 6,
            Some(AttributeName::Manipulation) => 5,
            Some(AttributeName::Appearance) => 4,
            Some(AttributeName::Perception) => 3,
            Some(AttributeName::Intelligence) => 2,
            Some(AttributeName::Wits) => 1,
        }
    }
}

impl FusedIterator for AttributeNameIter {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct Attribute {
    name: AttributeName,
    value: u8,
}

impl Attribute {
    pub fn name(&self) -> AttributeName {
        self.name
    }

    pub fn dots(&self) -> u8 {
        self.value
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
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
    pub fn get(&self, attribute_name: AttributeName) -> Attribute {
        Attribute {
            name: attribute_name,
            value: match attribute_name {
                AttributeName::Strength => self.strength,
                AttributeName::Dexterity => self.dexterity,
                AttributeName::Stamina => self.stamina,
                AttributeName::Charisma => self.charisma,
                AttributeName::Manipulation => self.manipulation,
                AttributeName::Appearance => self.appearance,
                AttributeName::Perception => self.perception,
                AttributeName::Intelligence => self.intelligence,
                AttributeName::Wits => self.wits,
            },
        }
    }

    pub fn set(&mut self, attribute_name: AttributeName, value: u8) -> Result<()> {
        if value < 1 {
            return Err(eyre!("Attributes must be 1 or more"));
        }

        let ptr = match attribute_name {
            AttributeName::Strength => &mut self.strength,
            AttributeName::Dexterity => &mut self.dexterity,
            AttributeName::Stamina => &mut self.stamina,
            AttributeName::Charisma => &mut self.charisma,
            AttributeName::Manipulation => &mut self.manipulation,
            AttributeName::Appearance => &mut self.appearance,
            AttributeName::Perception => &mut self.perception,
            AttributeName::Intelligence => &mut self.intelligence,
            AttributeName::Wits => &mut self.wits,
        };

        *ptr = value;
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = Attribute> + '_ {
        AttributesIter {
            attributes: self,
            name_iter: AttributeName::iter(),
        }
    }

    pub(crate) fn meets_prerequisite(&self, prerequisite: &AttributePrerequisite) -> bool {
        self.get(prerequisite.attribute_name).dots() >= prerequisite.dots
    }
}

struct AttributesIter<'a> {
    attributes: &'a Attributes,
    name_iter: AttributeNameIter,
}

impl<'a> Iterator for AttributesIter<'a> {
    type Item = Attribute;

    fn next(&mut self) -> Option<Self::Item> {
        let attribute_name = self.name_iter.next()?;
        Some(self.attributes.get(attribute_name))
    }
}

impl<'a> ExactSizeIterator for AttributesIter<'a> {
    fn len(&self) -> usize {
        self.name_iter.len()
    }
}

impl<'a> FusedIterator for AttributesIter<'a> {}
