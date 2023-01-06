use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) enum WeaponRange {
    ContactOnly,
    Throwable(RangeBand),
    Archery(RangeBand),
}

/// A distance measurement, loosely. 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RangeBand {
    /// Melee range, or within a step or two of it.
    Close,
    /// A short sprint away from Close range, within easy throwing distance.
    Short,
    /// Long enough that someone with a bow could probably shoot you before you
    /// can close the distance.
    Medium,
    /// Long enough that you'd have to volley an arrow rather than shooting 
    /// directly.
    Long,
    /// Outside of mundane bow range.
    Extreme,
}

/// The distance of an actual attack being made.
pub enum AttackRange {
    /// Hand-to-hand combat using the Brawl, Melee, or MartialArts
    /// abilities.
    Melee,
    /// Ranged attack using the Thrown or Archery abilities. 
    Ranged(RangeBand),
}