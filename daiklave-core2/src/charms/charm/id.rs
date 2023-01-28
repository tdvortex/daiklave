use serde::{Deserialize, Serialize};

/// The name of a Charm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
