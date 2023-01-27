use serde::{Deserialize, Serialize};

use super::artifact::ArtifactArmorId;

/// A unique identifier for a piece of armor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmorId<'source> {
    /// Nonmagical armor.
    Mundane(&'source str),
    /// Unique, magical armor.
    Artifact(ArtifactArmorId),
}
