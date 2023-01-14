mod no_brawl;
pub(crate) use no_brawl::DawnCasteAbilityNoBrawl;
use serde::{Deserialize, Serialize};

/// The abilities which a Dawn Solar can have as Caste abilities. Note that,
/// while Martial Arts can be a Supernal ability, it is not selectable as a
/// Caste ability. Instead, Brawl as a Caste ability implies that Martial Arts
/// is also a Caste ability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub enum DawnCasteAbility {
    /// Archery
    Archery,
    /// Awareness
    Awareness,
    /// Brawl (and Martial Arts)
    Brawl,
    /// Dodge
    Dodge,
    /// Melee
    Melee,
    /// Resistance
    Resistance,
    /// Thrown
    Thrown,
    /// War
    War,
}
