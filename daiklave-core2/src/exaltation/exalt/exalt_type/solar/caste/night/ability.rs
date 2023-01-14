use serde::{Deserialize, Serialize};

/// The Caste and Supernal ability options for the Night caste
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub enum NightAbility {
    /// Athletics
    Athletics,
    /// Awareness
    Awareness,
    /// Dodge
    Dodge,
    /// Investigation
    Investigation,
    /// Larceny
    Larceny,
    /// Ride
    Ride,
    /// Stealth
    Stealth,
    /// Socialize
    Socialize,
}
