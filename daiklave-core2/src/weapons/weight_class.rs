use serde::{Deserialize, Serialize};

/// The weight category of a weapon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WeaponWeightClass {
    /// Light
    Light,
    /// Medium
    Medium,
    /// Heavy
    Heavy,
}
