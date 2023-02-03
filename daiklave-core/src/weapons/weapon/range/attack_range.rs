use super::band::RangeBand;

/// The distance of an actual attack being made.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttackRange {
    /// Hand-to-hand combat using the Brawl, Melee, or MartialArts
    /// abilities.
    Melee,
    /// Ranged attack using the Thrown or Archery abilities.
    Ranged(RangeBand),
}
