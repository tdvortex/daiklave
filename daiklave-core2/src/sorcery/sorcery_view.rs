use crate::exaltation::SorceryViewSwitch;

use super::{
    archetype::SorceryArchetype, archetype_id::SorceryArchetypeId, circles::SorceryCircle,
    shaping_ritual::ShapingRitual, shaping_ritual_id::ShapingRitualId, spell::Spell,
    spell_id::SpellId,
};

/// A character's Sorcery abilities.
pub struct SorceryView<'view, 'source>(pub(crate) SorceryViewSwitch<'view, 'source>);

impl<'view, 'source> SorceryView<'view, 'source> {
    /// The details of a specific sorcerous archetype, if it exists.
    pub fn archetype(&self, id: SorceryArchetypeId) -> Option<&'source SorceryArchetype> {
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
