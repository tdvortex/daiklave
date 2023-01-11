use serde::{Deserialize, Serialize};

/// The strength level of the hearthstone (and its associated manse, if any).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum GeomancyLevel {
    /// The most common power level.
    Standard,
    /// Rare, exceptionally powerful hearthstones.
    Greater,
}
