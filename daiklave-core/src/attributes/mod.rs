mod attribute;
mod category;
mod error;
mod name;
mod set;

use std::num::NonZeroU8;

pub use attribute::Attribute;
pub use category::AttributeCategory;
pub use error::AttributeError;
pub use name::AttributeName;
use serde::{Deserialize, Serialize};
pub use set::SetAttribute;

use crate::CharacterMutationError;

/// Struct representing a character's nine core Attributes (Strength, Intelligence,
/// etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attributes {
    strength: NonZeroU8,
    dexterity: NonZeroU8,
    stamina: NonZeroU8,
    charisma: NonZeroU8,
    manipulation: NonZeroU8,
    appearance: NonZeroU8,
    perception: NonZeroU8,
    intelligence: NonZeroU8,
    wits: NonZeroU8,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            strength: NonZeroU8::new(1).unwrap(),
            dexterity: NonZeroU8::new(1).unwrap(),
            stamina: NonZeroU8::new(1).unwrap(),
            charisma: NonZeroU8::new(1).unwrap(),
            manipulation: NonZeroU8::new(1).unwrap(),
            appearance: NonZeroU8::new(1).unwrap(),
            perception: NonZeroU8::new(1).unwrap(),
            intelligence: NonZeroU8::new(1).unwrap(),
            wits: NonZeroU8::new(1).unwrap(),
        }
    }
}

impl Attributes {
    /// Get a specific attribute by its name.
    pub fn get(&self, attribute_name: AttributeName) -> Attribute {
        Attribute {
            name: attribute_name,
            dots: self.dots(attribute_name),
        }
    }

    /// Iterates over all attributes.
    pub fn iter(&self) -> impl Iterator<Item = Attribute> + '_ {
        [
            AttributeName::Strength,
            AttributeName::Dexterity,
            AttributeName::Stamina,
            AttributeName::Charisma,
            AttributeName::Manipulation,
            AttributeName::Appearance,
            AttributeName::Intelligence,
            AttributeName::Perception,
            AttributeName::Wits,
        ]
        .into_iter()
        .map(|attribute_name| self.get(attribute_name))
    }

    pub(crate) fn dots(&self, attribute_name: AttributeName) -> NonZeroU8 {
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
        if dots > 5 {
            return Err(CharacterMutationError::AttributeError(
                AttributeError::InvalidRating,
            ));
        }

        let nonzero = NonZeroU8::new(dots).ok_or(CharacterMutationError::AttributeError(
            AttributeError::InvalidRating,
        ))?;

        match attribute_name {
            AttributeName::Strength => {
                self.strength = nonzero;
            }
            AttributeName::Dexterity => {
                self.dexterity = nonzero;
            }
            AttributeName::Stamina => {
                self.stamina = nonzero;
            }
            AttributeName::Charisma => {
                self.charisma = nonzero;
            }
            AttributeName::Manipulation => {
                self.manipulation = nonzero;
            }
            AttributeName::Appearance => {
                self.appearance = nonzero;
            }
            AttributeName::Perception => {
                self.perception = nonzero;
            }
            AttributeName::Intelligence => {
                self.intelligence = nonzero;
            }
            AttributeName::Wits => {
                self.wits = nonzero;
            }
        }

        Ok(self)
    }
}
