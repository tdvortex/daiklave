use serde::{Deserialize, Serialize};

/// A weight category for armor.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum ArmorWeightClass {
    /// Light armor, no significant mobility restrictions.
    Light,
    /// Medium armor, the kind worn by most battle-ready soldiers.
    Medium,
    /// Heavy armor, expensive and clunky.
    Heavy,
}
