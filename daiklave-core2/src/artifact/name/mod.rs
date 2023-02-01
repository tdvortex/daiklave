
mod mutation;

pub(crate) use mutation::ArtifactNameMutation;

use crate::exaltation::exalt::essence::MotePoolName;

use super::AttuneArtifact;

/// The name of a magical creation (weapon, armor, warstrider, or wonder).
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ArtifactName<'source> {
    /// An artifact weapon's name.
    Weapon(&'source str),
    /// An artifact armor item's name.
    Armor(&'source str),
    /// A wonder's name.
    Wonder(&'source str),
}

impl<'source> ArtifactName<'source> {
    /// Attune to an artifact with this name using the specified mote pool.
    pub fn attune(&self, first: MotePoolName) -> AttuneArtifact {
        AttuneArtifact {
            artifact_name: (*self).into(),
            first,
        }
    }
}