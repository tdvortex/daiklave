use serde::{Deserialize, Serialize};

/// One of the three tiers of Sorcery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SorceryCircle {
    /// The first and lowest circle. Usable by everyone, including some mortals
    Terrestrial,
    /// The second circle, usable by Solars (and Abyssals and Infernals),
    /// Lunars, Sidereals, and Getimians.
    Celestial,
    /// The third and highest circle, usable only by the most skilled of the
    /// Solars.
    Solar,
}
