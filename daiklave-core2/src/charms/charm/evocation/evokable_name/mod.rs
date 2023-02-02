mod mutation;
pub(crate) use mutation::EvokableNameMutation;

use crate::artifact::ArtifactName;

use super::{
    builder::{EvocationBuilder, EvocationBuilderWithName},
    EvocationName,
};

/// The name of an item which is capable of having Evocations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvokableName<'source> {
    /// Hearthstones may have unlockable Evocations.
    Hearthstone(&'source str),
    /// Artifacts may have unlockable Evocations.
    Artifact(ArtifactName<'source>),
}

impl<'source> EvokableName<'source> {
    /// Starts a builder process to construct an evocation for this item.
    pub fn with_evocation(self, name: impl Into<EvocationName>) -> EvocationBuilderWithName {
        EvocationBuilder::evocation_of(self).name(name)
    }
}

impl<'source> From<ArtifactName<'source>> for EvokableName<'source> {
    fn from(name: ArtifactName<'source>) -> Self {
        Self::Artifact(name)
    }
}

impl<'source> From<&'source EvokableNameMutation> for EvokableName<'source> {
    fn from(name: &'source EvokableNameMutation) -> Self {
        match name {
            EvokableNameMutation::Hearthstone(name) => Self::Hearthstone(name.as_str()),
            EvokableNameMutation::Artifact(name) => Self::Artifact(name.into()),
        }
    }
}
