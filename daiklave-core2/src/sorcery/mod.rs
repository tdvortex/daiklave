mod archetype;
pub(crate) mod circles;
mod error;
mod spells;
pub use spells::Spells;

/// Properties of an individual Spell.
pub mod spell;

pub use archetype::{
    ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId, SorceryArchetypeMerit,
    SorceryArchetypeMeritId, SorceryArchetypeWithMerits,
};
pub use circles::{
    AddCelestialSorcery, AddSolarSorcery, AddTerrestrialSorcery, CelestialSpell, SolarSpell,
    SorceryCircle, TerrestrialSpell,
};
pub(crate) use error::SorceryError;

use crate::exaltation::ExaltationSorcery;

use self::spell::{Spell, SpellId};

/// A character's Sorcery abilities.
pub struct Sorcery<'view, 'source>(pub(crate) ExaltationSorcery<'view, 'source>);

impl<'view, 'source> Sorcery<'view, 'source> {
    /// The details of a specific sorcerous archetype, if it exists.
    pub fn archetype(
        &self,
        id: SorceryArchetypeId,
    ) -> Option<SorceryArchetypeWithMerits<'view, 'source>> {
        self.0.archetype(id)
    }

    /// Iterates over all sorcerous archetypes the character possesses by their Id.
    pub fn archetypes(&self) -> impl Iterator<Item = SorceryArchetypeId> + '_ {
        self.0.archetypes_iter()
    }

    /// The shaping ritual the character learned at a specific circle induction.
    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> Option<(ShapingRitualId, &'source ShapingRitual)> {
        self.0.shaping_ritual(circle)
    }

    /// The control spell the character learned at a specific circle induction.
    pub fn control_spell(&self, circle: SorceryCircle) -> Option<(SpellId, Spell<'source>)> {
        self.0.control_spell(circle)
    }

    /// Access the spells the sorcerer knows.
    pub fn spells(&self) -> Spells<'view, 'source> {
        Spells(self.0)
    }
}
