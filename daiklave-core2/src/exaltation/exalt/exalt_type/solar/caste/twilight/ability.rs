use serde::{Deserialize, Serialize};

/// The Caste and Supernal ability options for the Twilight caste
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub enum TwilightAbility {
    /// Bureaucracy
    Bureaucracy,
    /// Craft
    Craft,
    /// Integrity
    Integrity,
    /// Investigation
    Investigation,
    /// Linguistics
    Linguistics,
    /// Lore
    Lore,
    /// Medicine
    Medicine,
    /// Occult
    Occult,
}
