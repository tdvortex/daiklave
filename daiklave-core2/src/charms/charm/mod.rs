mod builder;
pub use builder::CharmBuilder;

/// Evocations of artifacts and hearthstones.
pub mod evocation;
mod id;
mod mutation;
mod spirit;
use crate::{
    exaltation::exalt::exalt_type::solar::charm::SolarCharm,
    martial_arts::MartialArtsCharm,
    sorcery::{CelestialSpell, SolarSpell, TerrestrialSpell},
};
pub use mutation::CharmMutation;

use self::{spirit::EclipseCharm, evocation::Evocation};

pub use id::CharmId;
pub use spirit::{SpiritCharmId, SpiritCharmKeyword};

/// A Charm possessed by a character.
pub enum Charm<'source> {
    /// A Spirit charm with the Eclipse keyword, purchasable by Eclipse caste
    /// Solars.
    Eclipse(&'source EclipseCharm),
    /// An Evocation of an artifact or hearthstone.
    Evocation(&'source Evocation),
    /// A Martial Arts charm for a specific style.
    MartialArts(&'source MartialArtsCharm),
    /// A Solar charm.
    Solar(&'source SolarCharm),
    /// A Spell of the Terrestrial Circle.
    TerrestrialSpell(&'source TerrestrialSpell),
    /// A Spell of the Celestial Circle.
    CelestialSpell(&'source CelestialSpell),
    /// A Spell of the Solar Circle.
    SolarSpell(&'source SolarSpell),
}

impl<'source> Charm<'source> {
    /// Begins construction of a new Charm.
    pub fn new(name: String) -> CharmBuilder {
        CharmBuilder {
            name,
            book_reference: None,
            summary: None,
        }
    }
}