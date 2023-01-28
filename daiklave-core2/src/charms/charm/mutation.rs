use crate::{
    exaltation::exalt::exalt_type::solar::charm::{SolarCharm, SolarCharmId},
    martial_arts::charm::{AddMartialArtsCharm},
    sorcery::spell::AddSpell,
};

use super::{
    evocation::{AddEvocation},
    spirit::EclipseCharm,
    SpiritCharmId,
};

/// A Charm to be added to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddCharm {
    /// A Spirit charm with the Eclipse keyword, purchasable by Eclipse caste
    /// Solars.
    Eclipse(SpiritCharmId, EclipseCharm),
    /// An Evocation of an artifact or hearthstone.
    Evocation(AddEvocation),
    /// A Martial Arts charm for a specific style.
    MartialArts(AddMartialArtsCharm),
    /// A Solar charm.
    Solar(SolarCharmId, SolarCharm),
    /// A Spell.
    Spell(AddSpell),
}
