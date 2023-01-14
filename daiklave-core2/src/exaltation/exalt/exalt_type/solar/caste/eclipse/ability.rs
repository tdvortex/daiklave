use serde::{Deserialize, Serialize};

/// The Caste and Supernal ability choices for Eclipse Castes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub enum EclipseAbility {
    /// Bureaucracy
    Bureaucracy,
    /// Larceny
    Larceny,
    /// Linguistics
    Linguistics,
    /// Occult
    Occult,
    /// Presence
    Presence,
    /// Ride
    Ride,
    /// Sail
    Sail,
    /// Socialize
    Socialize,
}
