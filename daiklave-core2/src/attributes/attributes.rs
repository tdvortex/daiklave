use serde::{Deserialize, Serialize};

use crate::CharacterMutationError;

use super::{attribute_name::AttributeName, SetAttributesError};

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
    pub fn dots(&self, attribute_name: AttributeName) -> u8 {
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

    pub(crate) fn set_dots(
        &mut self,
        attribute_name: AttributeName,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if !(1..=5).contains(&dots) {
            Err(CharacterMutationError::SetAttributesError(
                SetAttributesError::InvalidRating(dots),
            ))
        } else {
            match attribute_name {
                AttributeName::Strength => {
                    self.strength = dots;
                }
                AttributeName::Dexterity => {
                    self.dexterity = dots;
                }
                AttributeName::Stamina => {
                    self.stamina = dots;
                }
                AttributeName::Charisma => {
                    self.charisma = dots;
                }
                AttributeName::Manipulation => {
                    self.manipulation = dots;
                }
                AttributeName::Appearance => {
                    self.appearance = dots;
                }
                AttributeName::Perception => {
                    self.perception = dots;
                }
                AttributeName::Intelligence => {
                    self.intelligence = dots;
                }
                AttributeName::Wits => {
                    self.wits = dots;
                }
            }
            Ok(self)
        }
    }
}