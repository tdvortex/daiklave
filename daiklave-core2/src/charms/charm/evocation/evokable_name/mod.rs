mod mutation;
pub use mutation::EvokableNameMutation;

use crate::artifact::ArtifactName;

/// The name of an item which is capable of having Evocations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvokableName<'source> {
    /// Hearthstones may have unlockable Evocations.
    Hearthstone(&'source str),
    /// Artifacts may have unlockable Evocations.
    Artifact(ArtifactName<'source>),
}
