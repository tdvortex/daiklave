use serde::{Serialize, Deserialize};

/// The level of an Intimacy. Can be converted into a u8 using the From trait.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum IntimacyLevel {
    /// A Minor Intimacy, something that the character cares about but which is
    /// not always front-of-mind.
    Minor,
    /// A Major Intimacy, something which is an important factor in any
    /// decision it might relate to.
    Major,
    /// A Defining Intimacy, something which your character would die for.
    Defining,
}

impl From<IntimacyLevel> for u8 {
    fn from(level: IntimacyLevel) -> Self {
        match level {
            IntimacyLevel::Minor => 2,
            IntimacyLevel::Major => 3,
            IntimacyLevel::Defining => 4,
        }
    }
}