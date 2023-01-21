use serde::{Serialize, Deserialize};

/// Keywords for Spells.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub enum SpellKeyword {
    /// Aggravated damage-dealing
    Aggravated,
    /// Only applies to Decisive attacks
    DecisiveOnly,    
    /// Cannot be used while crashed
    Perilous,
    /// Mind-affecting Charms
    Psyche,
}