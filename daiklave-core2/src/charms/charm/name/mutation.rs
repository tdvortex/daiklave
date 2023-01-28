use serde::{Deserialize, Serialize};

use crate::sorcery::spell::SpellName;

/// The name of a Charm, to be used in adding or removing Charms.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharmNameMutation {
    /// A Spirit charm.
    Spirit(String),
    /// An Evocation of an artifact or hearthstone.
    Evocation(String),
    /// A Martial Arts charm for a specific style.
    MartialArts(String),
    /// A Solar charm.
    Solar(String),
    /// A Spell.
    Spell(SpellName),
}
