use serde::{Deserialize, Serialize};

use super::{ArtifactWeaponId};

/// The Id for a weapon.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum WeaponId<'source> {
    /// All characters have the Unarmed weapon for free, and it cannot
    /// be removed.
    Unarmed,
    /// A mundane weapon without artifact traits.
    Mundane(&'source str),
    /// A unique magical weapon.
    Artifact(ArtifactWeaponId),
}
