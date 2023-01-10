use crate::{artifact::ArtifactId, hearthstones::HearthstoneId};

/// The Id for an item which is capable of having Evocations.
pub enum _EvokableItemId {
    /// Hearthstones may have unlockable Evocations.
    Hearthstone(HearthstoneId),
    /// Artifacts may have unlockable Evocations.
    Artifact(ArtifactId),
}
