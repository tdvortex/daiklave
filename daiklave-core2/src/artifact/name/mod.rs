
mod mutation;

pub(crate) use mutation::ArtifactNameMutation;

use crate::{exaltation::exalt::essence::MotePoolName, hearthstones::hearthstone::{HearthstoneName, SlotHearthstone}};

use super::{AttuneArtifact, RemoveArtifact};

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
    pub fn attune(self, first: MotePoolName) -> AttuneArtifact {
        AttuneArtifact {
            artifact_name: self.into(),
            first,
        }
    }
    
    /// Slot a hearthstone into this artifact.
    pub fn slot_hearthstone(self, hearthstone_name: impl Into<HearthstoneName>) -> SlotHearthstone {
        Into::<HearthstoneName>::into(hearthstone_name).slot_into(self)
    }

    /// Remove this artifact from a character.
    pub fn remove(self) -> RemoveArtifact {
        RemoveArtifact(self.into())
    }
}