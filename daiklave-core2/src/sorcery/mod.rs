mod add;
mod remove;
pub use add::AddSorcery;
pub use remove::RemoveSorcery;
mod archetype;
/// A builder path to construct a sorcerous intiation.
pub mod builder;
pub(crate) mod circles;
mod error;
mod spells;
pub use spells::Spells;

/// Properties of an individual Spell.
pub mod spell;

pub use archetype::{
    AddShapingRitual, ShapingRitual, SorceryArchetypeMerit,
    SorceryArchetypeName, SorceryArchetypeWithMerits,
};
pub use circles::{
    AddCelestialSorcery, AddSolarSorcery, AddTerrestrialSorcery, CelestialSpell, SolarSpell,
    SorceryCircle, TerrestrialSpell,
};
pub(crate) use error::SorceryError;

use crate::exaltation::ExaltationSorcery;

use self::{builder::SorceryBuilder, spell::Spell, archetype::SorceryArchetype};

/// A character's Sorcery abilities.
pub struct Sorcery<'view, 'source>(pub(crate) ExaltationSorcery<'view, 'source>);

impl<'view, 'source> Sorcery<'view, 'source> {
    /// Starts building a new circle of Sorcery to add to a character.
    pub fn builder() -> SorceryBuilder {
        SorceryBuilder
    }

    /// The details of a specific sorcerous archetype, if it exists.
    pub fn archetype(&self, name: &str) -> Option<SorceryArchetypeWithMerits<'view, 'source>> {
        self.0.archetype(name)
    }

    /// Iterates over all sorcerous archetypes the character possesses by their name.
    pub fn archetypes(&self) -> impl Iterator<Item = &'source str> + '_ {
        self.0.archetypes_iter()
    }

    /// The shaping ritual the character learned at a specific circle induction.
    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> Option<(&'source str, &'source ShapingRitual)> {
        self.0.shaping_ritual(circle)
    }

    /// The control spell the character learned at a specific circle induction.
    pub fn control_spell(&self, circle: SorceryCircle) -> Option<Spell<'source>> {
        self.0.control_spell(circle)
    }

    /// Access the spells the sorcerer knows.
    pub fn spells(&self) -> Spells<'view, 'source> {
        Spells(self.0)
    }
}
