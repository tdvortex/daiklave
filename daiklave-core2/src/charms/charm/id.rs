use serde::{Deserialize, Serialize};

use super::{spirit::SpiritCharmId};

/// The Id of a Charm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharmId<'source> {
    /// A Spirit charm.
    Spirit(SpiritCharmId),
    /// An Evocation of an artifact or hearthstone.
    Evocation(&'source str),
    /// A Martial Arts charm for a specific style.
    MartialArts(&'source str),
    /// A Solar charm.
    Solar(&'source str),
    /// A Spell.
    Spell(&'source str),
}
