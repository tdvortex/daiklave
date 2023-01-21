mod evocation;
mod id;
mod mutation;
mod spirit;
pub use mutation::CharmMutation;
use crate::{martial_arts::MartialArtsCharm, sorcery::{TerrestrialSpell, SolarSpell, CelestialSpell}, exaltation::exalt::exalt_type::solar::charm::SolarCharm};

use self::{spirit::EclipseCharm, evocation::Evocation};

pub use id::CharmId;

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
    pub(crate) fn as_memo(&self) -> CharmMutation {
        match self {
            Charm::Eclipse(view) => CharmMutation::Eclipse((*view).to_owned()),
            Charm::Evocation(view) => CharmMutation::Evocation((*view).to_owned()),
            Charm::MartialArts(view) => CharmMutation::MartialArts((*view).to_owned()),
            Charm::Solar(view) => CharmMutation::Solar((*view).to_owned()),
            Charm::TerrestrialSpell(view) => CharmMutation::TerrestrialSpell((*view).to_owned()),
            Charm::CelestialSpell(view) => CharmMutation::CelestialSpell((*view).to_owned()),
            Charm::SolarSpell(view) => CharmMutation::SolarSpell((*view).to_owned()),
        }
    }
}