use serde::{Deserialize, Serialize};

use super::wonders::WonderId;

/// The name of a magical creation (weapon, armor, warstrider, or wonder).
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum ArtifactName {
    /// An artifact weapon's name.
    Weapon(String),
    /// An artifact armor item's name.
    Armor(String),
    /// A wonder's Id.
    Wonder(WonderId),
}
