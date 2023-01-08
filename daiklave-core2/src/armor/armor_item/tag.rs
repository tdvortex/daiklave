use serde::{Deserialize, Serialize};

/// A tag detailing a secondary propety of an armor item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmorTag {
    /// The armor can be worn under clothes.
    Concealable,
    /// The armor's mobility penalty doesn't apply to Stealth checks.
    Silent,
    /// The armor has some unique property, see the rulebook for details.
    Special,
}
