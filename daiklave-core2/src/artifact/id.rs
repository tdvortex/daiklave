use serde::{Deserialize, Serialize};

use super::wonders::WonderId;

/// The Id for a magical creation (weapon, armor, warstrider, or wonder).
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum ArtifactId<'source> {
    /// An artifact weapon's name.
    Weapon(&'source str),
    /// An artifact armor item's name.
    Armor(&'source str),
    /// A wonder's Id.
    Wonder(WonderId),
}
