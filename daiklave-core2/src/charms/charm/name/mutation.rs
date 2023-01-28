use serde::{Deserialize, Serialize};

use crate::{sorcery::spell::SpellName, charms::charm::{spirit::SpiritCharmName, evocation::EvocationName}, martial_arts::charm::MartialArtsCharmName, exaltation::exalt::exalt_type::solar::charm::SolarCharmName};

/// The name of a Charm, to be used in adding or removing Charms.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharmNameMutation {
    /// A Spirit charm.
    Spirit(SpiritCharmName),
    /// An Evocation of an artifact or hearthstone.
    Evocation(EvocationName),
    /// A Martial Arts charm for a specific style.
    MartialArts(MartialArtsCharmName),
    /// A Solar charm.
    Solar(SolarCharmName),
    /// A Spell.
    Spell(SpellName),
}
