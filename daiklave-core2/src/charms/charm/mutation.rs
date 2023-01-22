use crate::{
    exaltation::exalt::exalt_type::solar::charm::{SolarCharm, SolarCharmId}, martial_arts::charm::{MartialArtsCharm, MartialArtsCharmId},
    sorcery::spell::{SpellMutation, SpellId},
};

use super::{evocation::{Evocation, EvocationId}, spirit::EclipseCharm, SpiritCharmId};

/// A Charm to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharmMutation {
    /// A Spirit charm with the Eclipse keyword, purchasable by Eclipse caste
    /// Solars.
    Eclipse(SpiritCharmId, EclipseCharm),
    /// An Evocation of an artifact or hearthstone.
    Evocation(EvocationId, Evocation),
    /// A Martial Arts charm for a specific style.
    MartialArts(MartialArtsCharmId, MartialArtsCharm),
    /// A Solar charm.
    Solar(SolarCharmId, SolarCharm),
    /// A Spell.
    Spell(SpellId, SpellMutation),
}
