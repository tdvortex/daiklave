use serde::{Serialize, Deserialize};

/// Represents a (non-neutral) relationship to a Magic Material for an
/// Exalt type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Sonance {
    /// The Exalt is resonant with the magic material and can draw extra power
    /// from its Evocations.
    Resonant,
    /// The Exalt is dissonant with the magic material and may struggle to draw
    /// the full power of its Evocations.
    Dissonant,
}