mod archetype;
pub(crate) mod circles;
mod error;
mod spell;

pub use archetype::{SorceryArchetype, SorceryArchetypeMerit, SorceryArchetypeMeritId, SorceryArchetypeId, ShapingRitual, ShapingRitualId, SorceryArchetypeWithMerits};
pub(crate) use error::SorceryError;
pub use spell::{Spell, SpellId};
pub use circles::{CelestialSpell, SolarSpell, SorceryCircle, TerrestrialSpell};

use crate::exaltation::ExaltationSorcery;

/// A character's Sorcery abilities.
pub struct Sorcery<'view, 'source>(pub(crate) ExaltationSorcery<'view, 'source>);

impl<'view, 'source> Sorcery<'view, 'source> {
    /// The details of a specific sorcerous archetype, if it exists.
    pub fn archetype(&self, id: SorceryArchetypeId) -> Option<SorceryArchetypeWithMerits> {
        self.0.archetype(id)
    }

    /// The shaping ritual the character learned at a specific circle induction.
    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> Option<(ShapingRitualId, &'source ShapingRitual)> {
        self.0.shaping_ritual(circle)
    }

    /// The control spell the character learned at a specific circle induction.
    pub fn control_spell(&self, circle: SorceryCircle) -> Option<(SpellId, &'source Spell)> {
        self.0.control_spell(circle)
    }
}
