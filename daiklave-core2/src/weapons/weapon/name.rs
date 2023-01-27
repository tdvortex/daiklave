use serde::{Serialize, Deserialize};

use super::ArtifactWeaponId;

/// The name of a weapon.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum WeaponName {
    /// All characters have the Unarmed weapon for free, and it cannot
    /// be removed.
    Unarmed,
    /// A mundane weapon without artifact traits.
    Mundane(String),
    /// A unique magical weapon.
    Artifact(ArtifactWeaponId),
}
