use super::{AddCelestialSorcery, AddSolarSorcery, AddTerrestrialSorcery};

/// A mutation to add Sorcery to a character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddSorcery {
    /// Adds the Terrestrial circle of sorcery.
    Terrestrial(AddTerrestrialSorcery),
    /// Adds the Celestial circle of sorcery.
    Celestial(AddCelestialSorcery),
    /// Adds the Solar circle of sorcery.
    Solar(AddSolarSorcery),
}
