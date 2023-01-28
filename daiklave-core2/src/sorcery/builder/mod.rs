mod celestial;
mod solar;
mod terrestrial;
pub use celestial::{
    CelestialSorceryBuilder, CelestialSorceryBuilderWithArchetype,
    CelestialSorceryBuilderWithShapingRitual,
};
pub use solar::{
    SolarSorceryBuilder, SolarSorceryBuilderWithArchetype, SolarSorceryBuilderWithShapingRitual,
};
pub use terrestrial::{
    TerrestrialSorceryBuilder, TerrestrialSorceryBuilderWithArchetype,
    TerrestrialSorceryBuilderWithShapingRitual,
};

/// A builder to create a sorcerer's initiation into a new level of srocery.
pub struct SorceryBuilder;

impl SorceryBuilder {
    /// Builds a Terrestrial circle initiation.
    pub fn terrestrial(self) -> TerrestrialSorceryBuilder {
        TerrestrialSorceryBuilder
    }

    /// Builds a Celestial circle initiation.
    pub fn celestial(self) -> CelestialSorceryBuilder {
        CelestialSorceryBuilder
    }

    /// Builds a Solar circle initiation.
    pub fn solar(self) -> SolarSorceryBuilder {
        SolarSorceryBuilder
    }
}
