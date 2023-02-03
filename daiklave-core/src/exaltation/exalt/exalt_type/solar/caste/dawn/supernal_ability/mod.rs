mod layout;
pub(crate) use layout::DawnSupernalLayout;

use serde::{Deserialize, Serialize};

/// The abilities which a Dawn may have as a Supernal ability. Martial Arts
/// may be a Supernal ability only if Brawl is a Caste ability, but only one of
/// Brawl or Martial Arts can be Supernal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DawnSupernalAbility {
    /// Archery
    Archery,
    /// Awareness
    Awareness,
    /// Brawl
    Brawl,
    /// Dodge
    Dodge,
    /// Martial Arts
    MartialArts,
    /// Melee
    Melee,
    /// Resistance
    Resistance,
    /// Thrown
    Thrown,
    /// War
    War,
}
