use serde::{Deserialize, Serialize};

use crate::{
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
    Solar(String),
    /// A Spell.
    Spell(SpellName),
}
