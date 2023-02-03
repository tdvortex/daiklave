pub(crate) mod celestial;
pub(crate) mod solar;
pub(crate) mod sorcery_circle;
pub(crate) mod terrestrial;

pub use celestial::{AddCelestialSorcery, CelestialSpell};
pub use solar::{AddSolarSorcery, SolarSpell};
pub use sorcery_circle::SorceryCircle;
pub use terrestrial::{AddTerrestrialSorcery, TerrestrialSpell};
