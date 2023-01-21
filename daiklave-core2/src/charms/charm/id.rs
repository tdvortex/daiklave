use serde::{Deserialize, Serialize};

use crate::{
    exaltation::exalt::exalt_type::solar::charm::SolarCharmId, martial_arts::MartialArtsCharmId,
    sorcery::spell::SpellId,
};

use super::{evocation::EvocationId, spirit::SpiritCharmId};

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
    Spell(SpellId),
}