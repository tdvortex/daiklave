use serde::{Deserialize, Serialize};

use crate::{
    exaltation::exalt::exalt_type::solar::charm::SolarCharmId,
    sorcery::spell::SpellName,
};

use super::{SpiritCharmId};

/// The name of a Charm.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharmName {
    /// A Spirit charm.
    Spirit(SpiritCharmId),
    /// An Evocation of an artifact or hearthstone.
    Evocation(String),
    /// A Martial Arts charm for a specific style.
    MartialArts(String),
    /// A Solar charm.
    Solar(SolarCharmId),
    /// A Spell.
    Spell(SpellName),
}
