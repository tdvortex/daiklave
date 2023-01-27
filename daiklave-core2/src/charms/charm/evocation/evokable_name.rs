use serde::{Deserialize, Serialize};

use crate::{artifact::ArtifactName, hearthstones::HearthstoneId};

/// The name of an item which is capable of having Evocations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvokableName {
    /// Hearthstones may have unlockable Evocations.
    Hearthstone(HearthstoneId),
    /// Artifacts may have unlockable Evocations.
    Artifact(ArtifactName),
}
