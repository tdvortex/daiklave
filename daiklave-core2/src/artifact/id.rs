use serde::{Deserialize, Serialize};

use crate::{weapons::weapon::ArtifactWeaponId, armor::armor_item::artifact::ArtifactArmorId};

/// The Id for a magical creation (weapon, armor, warstrider, or wonder).
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum ArtifactId {
    /// An artifact weapon's Id.
    Weapon(ArtifactWeaponId),
    /// An artifact armor item's Id.
    Armor(ArtifactArmorId),
}
