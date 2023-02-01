mod mutation;
pub(crate) use mutation::CharmNameMutation;

use super::RemoveCharm;

/// The name of a Charm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharmName<'source> {
    /// A Spirit charm.
    Spirit(&'source str),
    /// An Evocation of an artifact or hearthstone.
    Evocation(&'source str),
    /// A Martial Arts charm for a specific style.
    MartialArts(&'source str),
    /// A Solar charm.
    Solar(&'source str),
    /// A Spell.
    Spell(&'source str),
}

impl<'source> CharmName<'source> {
    /// Creates a mutation to remove this charm from the character.
    pub fn remove(self) -> RemoveCharm {
        RemoveCharm(self.into())
    }
}