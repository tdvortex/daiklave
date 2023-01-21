use crate::{martial_arts::MartialArtsCharm, sorcery::{TerrestrialSpell, CelestialSpell, SolarSpell}, exaltation::exalt::exalt_type::solar::charm::SolarCharm};

use super::{Charm, spirit::EclipseCharm, evocation::Evocation};

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

impl<'source> CharmMutation {
    pub(crate) fn as_ref(&'source self) -> Charm<'source> {
        match self {
            CharmMutation::Eclipse(memo) => Charm::Eclipse(memo),
            CharmMutation::Evocation(memo) => Charm::Evocation(memo),
            CharmMutation::MartialArts(memo) => Charm::MartialArts(memo),
            CharmMutation::Solar(memo) => Charm::Solar(memo),
            CharmMutation::TerrestrialSpell(memo) => Charm::TerrestrialSpell(memo),
            CharmMutation::CelestialSpell(memo) => Charm::CelestialSpell(memo),
            CharmMutation::SolarSpell(memo) => Charm::SolarSpell(memo),
        }
    }
}