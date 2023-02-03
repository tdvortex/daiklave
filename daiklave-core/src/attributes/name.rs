use serde::{Deserialize, Serialize};

use super::{AttributeError, SetAttribute};

/// The nine attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl AttributeName {
    /// Creates a new SetAttribute mutation to set this attribute's dots.
    pub fn set_dots(&self, dots: u8) -> Result<SetAttribute, AttributeError> {
        SetAttribute::new(*self, dots)
    }
}
