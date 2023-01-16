use serde::{Deserialize, Serialize};

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
