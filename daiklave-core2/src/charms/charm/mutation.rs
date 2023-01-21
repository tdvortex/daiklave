use crate::{
    exaltation::exalt::exalt_type::solar::charm::SolarCharm,
    martial_arts::MartialArtsCharm,
    sorcery::{CelestialSpell, SolarSpell, TerrestrialSpell},
};

use super::{evocation::Evocation, spirit::EclipseCharm};

/// A Charm to be added to a character.
pub enum CharmMutation {
    /// A Spirit charm with the Eclipse keyword, purchasable by Eclipse caste
    /// Solars.
    Eclipse(EclipseCharm),
    /// An Evocation of an artifact or hearthstone.
    Evocation(Evocation),
    /// A Martial Arts charm for a specific style.
    MartialArts(MartialArtsCharm),
    /// A Solar charm.
    Solar(SolarCharm),
    /// A Spell of the Terrestrial Circle.
    TerrestrialSpell(TerrestrialSpell),
    /// A Spell of the Celestial Circle.
    CelestialSpell(CelestialSpell),
    /// A Spell of the Solar Circle.
    SolarSpell(SolarSpell),
}
