use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{Character, CharacterMutationError};

/// Struct representing a character's nine core Attributes (Strength, Intelligence,
/// etc.)
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

/// The nine attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeName {
    /// Strength
    Strength,
    /// Dexterity
    Dexterity,
    /// Stamina
    Stamina,
    /// Charisma
    Charisma,
    /// Manipulation
    Manipulation,
    /// Appearance
    Appearance,
    /// Perception
    Perception,
    /// Intelligence
    Intelligence,
    /// Wits
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
            wits: 1,
        }
    }
}

impl Attributes {
    /// Returns the dot value for the specific attribute.
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
    /// Gets a struct reference for the character's attributes.
    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    /// Validates that the requested dot level is an appropriate attribute
    /// rating. Attributes must be between 1 and 5 for all player characters.
    pub fn check_set_attribute(
        &self,
        _attribute_name: AttributeName,
        dots: u8,
    ) -> Result<(), CharacterMutationError> {
        if !(1..=5).contains(&dots) {
            Err(CharacterMutationError::SetAttributesError(
                SetAttributesError::InvalidRating(dots),
            ))
        } else {
            Ok(())
        }
    }

    /// Sets the specified attribute name to the specified dot rating.
    pub fn set_attribute(
        &mut self,
        attribute_name: AttributeName,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_attribute(attribute_name, dots)?;
        match attribute_name {
            AttributeName::Strength => {
                self.attributes.strength = dots;
            }
            AttributeName::Dexterity => {
                self.attributes.dexterity = dots;
            }
            AttributeName::Stamina => {
                self.attributes.stamina = dots;
            }
            AttributeName::Charisma => {
                self.attributes.charisma = dots;
            }
            AttributeName::Manipulation => {
                self.attributes.manipulation = dots;
            }
            AttributeName::Appearance => {
                self.attributes.appearance = dots;
            }
            AttributeName::Perception => {
                self.attributes.perception = dots;
            }
            AttributeName::Intelligence => {
                self.attributes.intelligence = dots;
            }
            AttributeName::Wits => {
                self.attributes.wits = dots;
            }
        }
        Ok(self)
    }
}
