use serde::{Serialize, Deserialize};

use super::{BaseArmorId, artifact::ArtifactArmorId};

/// A unique identifier for a piece of armor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmorId {
    /// Nonmagical armor.
    Mundane(BaseArmorId),
    /// Unique, magical armor.
    Artifact(ArtifactArmorId),
}