use serde::{Deserialize, Serialize};

use crate::armor::armor_item::artifact::ArtifactArmorId;

use super::wonders::WonderId;

/// The name of a magical creation (weapon, armor, warstrider, or wonder).
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum ArtifactName {
    /// An artifact weapon's Id.
    Weapon(String),
    /// An artifact armor item's Id.
    Armor(ArtifactArmorId),
    /// A wonder's Id.
    Wonder(WonderId),
}
