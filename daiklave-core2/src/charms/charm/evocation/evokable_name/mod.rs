mod mutation;
pub use mutation::EvokableNameMutation;

use crate::artifact::ArtifactName;

use super::{EvocationName, builder::{EvocationBuilderWithName, EvocationBuilder}};

/// The name of an item which is capable of having Evocations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvokableName<'source> {
    /// Hearthstones may have unlockable Evocations.
    Hearthstone(&'source str),
    /// Artifacts may have unlockable Evocations.
    Artifact(ArtifactName<'source>),
}

impl<'source> EvokableName<'source> {
    pub fn evocation(&self, name: impl Into<EvocationName>) -> EvocationBuilderWithName {
        EvocationBuilder::evocation_of(self.to_owned()).name(name)
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