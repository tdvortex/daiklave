use serde::{Serialize, Deserialize};

use crate::{martial_arts::MartialArtsCharmId, exaltation::exalt::exalt_type::solar::charm::SolarCharmId, sorcery::SpellId};

use super::{spirit::SpiritCharmId, evocation::EvocationId};

/// The Id of a Charm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharmId {
    /// A Spirit charm.
    Spirit(SpiritCharmId),
    /// An Evocation of an artifact or hearthstone. 
    Evocation(EvocationId),
    /// A Martial Arts charm for a specific style.
    MartialArts(MartialArtsCharmId),
    /// A Solar charm.
    Solar(SolarCharmId),
    /// A Spell.
    Spell(SpellId)
}