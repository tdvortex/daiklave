use serde::{Deserialize, Serialize};

/// The Caste and Supernal ability options for the Zenith caste
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub enum ZenithAbility {
    /// Athletics
    Athletics,
    /// Integrity
    Integrity,
    /// Performance
    Performance,
    /// Lore
    Lore,
    /// Presence
    Presence,
    /// Resistance
    Resistance,
    /// Survival
    Survival,
    /// War
    War,
}
