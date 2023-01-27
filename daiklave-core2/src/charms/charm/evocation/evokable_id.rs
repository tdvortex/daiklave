use crate::{artifact::ArtifactId, hearthstones::HearthstoneId};

/// The Id for an item which is capable of having Evocations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvokableId<'source> {
    /// Hearthstones may have unlockable Evocations.
    Hearthstone(HearthstoneId),
    /// Artifacts may have unlockable Evocations.
    Artifact(ArtifactId<'source>),
}
