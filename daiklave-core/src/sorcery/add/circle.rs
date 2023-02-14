use serde::{Serialize, Deserialize};

use crate::sorcery::{AddCelestialSorcery, AddSolarSorcery, AddTerrestrialSorcery};

/// Which Sorcery circle is being added.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AddSorceryCircle {
    /// Adds the Terrestrial circle of sorcery.
    Terrestrial(AddTerrestrialSorcery),
    /// Adds the Celestial circle of sorcery.
    Celestial(AddCelestialSorcery),
    /// Adds the Solar circle of sorcery.
    Solar(AddSolarSorcery),
}
