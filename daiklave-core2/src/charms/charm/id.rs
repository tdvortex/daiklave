use serde::{Deserialize, Serialize};

use crate::{
    exaltation::exalt::exalt_type::solar::charm::SolarCharmId,
};

use super::{evocation::EvocationId, spirit::SpiritCharmId};

/// The Id of a Charm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharmId<'source> {
    /// A Spirit charm.
    Spirit(SpiritCharmId),
    /// An Evocation of an artifact or hearthstone.
    Evocation(EvocationId),
    /// A Martial Arts charm for a specific style.
    MartialArts(&'source str),
    /// A Solar charm.
    Solar(SolarCharmId),
    /// A Spell.
    Spell(&'source str),
}
