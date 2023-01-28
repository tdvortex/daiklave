use serde::{Deserialize, Serialize};

use crate::artifact::ArtifactNameMutation;

/// The name of an item which is capable of having Evocations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvokableNameMutation {
    /// Hearthstones may have unlockable Evocations.
    Hearthstone(String),
    /// Artifacts may have unlockable Evocations.
    Artifact(ArtifactNameMutation),
}
