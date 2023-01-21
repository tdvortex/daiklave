use serde::{Deserialize, Serialize};

use crate::{artifact::ArtifactId, hearthstones::HearthstoneId};

/// The Id for an item which is capable of having Evocations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvokableId {
    /// Hearthstones may have unlockable Evocations.
    Hearthstone(HearthstoneId),
    /// Artifacts may have unlockable Evocations.
    Artifact(ArtifactId),
}
