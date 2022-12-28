use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::{Character, CharacterMutationError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            wits: 1
        }
    }
}

impl Attributes {
    pub fn get_dots(&self, attribute_name: AttributeName) -> u8 {
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
}

#[derive(Debug, Error)]
pub enum SetAttributesError {
    #[error("Attributes must be between 1 and 5, {0} is invalid")]
    InvalidRating(u8),
}

impl Character {
    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    pub fn check_set_attribute(&self, attribute_name: AttributeName, dots: u8) -> Result<(), CharacterMutationError> {
        if dots < 1 || dots > 5 {
            Err(CharacterMutationError::SetAttributesError(SetAttributesError::InvalidRating(dots)))
        } else {
            Ok(())
        }
    }

    pub fn set_attribute(&mut self, attribute_name: AttributeName, dots: u8) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_attribute(attribute_name, dots)?;
        match attribute_name {
            AttributeName::Strength => {self.attributes.strength = dots;}
            AttributeName::Dexterity => {self.attributes.dexterity = dots;}
            AttributeName::Stamina => {self.attributes.stamina = dots;}
            AttributeName::Charisma => {self.attributes.charisma = dots;}
            AttributeName::Manipulation => {self.attributes.manipulation = dots;}
            AttributeName::Appearance => {self.attributes.appearance = dots;}
            AttributeName::Perception => {self.attributes.perception = dots;}
            AttributeName::Intelligence => {self.attributes.intelligence = dots;}
            AttributeName::Wits => {self.attributes.wits = dots;}
        }
        Ok(self)
    }
}