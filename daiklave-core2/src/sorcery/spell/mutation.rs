use crate::sorcery::{CelestialSpell, SolarSpell, TerrestrialSpell};

/// A Spell, grouped by its Circle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellMutation {
    /// The First Circle of spells, accessible to all Exalts and some mortals.
    Terrestrial(TerrestrialSpell),
    /// The Second Circle of spells, accessible to Solars, Lunars, and
    /// Sidereals.
    Celestial(CelestialSpell),
    /// The Third Circle of spells, accessible only to the Chosen of the
    /// Unconquered Sun.
    Solar(SolarSpell),
}

