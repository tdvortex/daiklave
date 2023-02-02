mod circle;
use crate::CharacterMutation;

pub use self::circle::AddSorceryCircle;

use super::{builder::{TerrestrialSorceryBuilder, CelestialSorceryBuilder, SolarSorceryBuilder}, AddTerrestrialSorcery, AddCelestialSorcery, AddSolarSorcery};

/// A mutation to add Sorcery to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddSorcery(pub Box<AddSorceryCircle>);

impl AddSorcery {
    /// Starts a builder to add the terrestrial circle of sorcery.
    pub fn terrestrial_circle() -> TerrestrialSorceryBuilder {
        TerrestrialSorceryBuilder
    }

    /// Starts a builder to add the celestial circle of sorcery.
    pub fn celestial_circle() -> CelestialSorceryBuilder {
        CelestialSorceryBuilder
    }

    /// Starts a builder to add the solar circle of sorcery.
    pub fn solar_circle() -> SolarSorceryBuilder {
        SolarSorceryBuilder
    }
}

impl From<AddTerrestrialSorcery> for AddSorcery {
    fn from(add_terrestrial_sorcery: AddTerrestrialSorcery) -> Self {
        Self(Box::new(AddSorceryCircle::Terrestrial(add_terrestrial_sorcery)))
    }
}

impl From<AddCelestialSorcery> for AddSorcery {
    fn from(add_celestial_sorcery: AddCelestialSorcery) -> Self {
        Self(Box::new(AddSorceryCircle::Celestial(add_celestial_sorcery)))
    }
}

impl From<AddSolarSorcery> for AddSorcery {
    fn from(add_solar_sorcery: AddSolarSorcery) -> Self {
        Self(Box::new(AddSorceryCircle::Solar(add_solar_sorcery)))
    }
}

impl From<AddSorcery> for CharacterMutation {
    fn from(add_sorcery: AddSorcery) -> Self {
        Self::AddSorcery(add_sorcery)
    }
}
