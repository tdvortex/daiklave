use crate::exaltation::SorcerySwitchMemo;

use super::{SorceryArchetypeId, SorceryArchetype, circles::SorceryCircle, ShapingRitualId, ShapingRitual, SpellId, Spell};

/// A character's Sorcery abilities.
pub struct SorceryMemo<'char>(pub(crate) SorcerySwitchMemo<'char>);

impl<'char> SorceryMemo<'char> {
    /// The details of a specific sorcerous archetype, if it exists.
    pub fn archetype(&'char self, id: SorceryArchetypeId) -> Option<&'char SorceryArchetype> {
        self.0.archetype(id)
    }

    /// The shaping ritual the character learned at a specific circle induction.
    pub fn shaping_ritual(
        &'char self,
        circle: SorceryCircle,
    ) -> Option<(ShapingRitualId, &'char ShapingRitual)> {
        self.0.shaping_ritual(circle)
    }

    /// The control spell the character learned at a specific circle induction.
    pub fn control_spell(&'char self, circle: SorceryCircle) -> Option<(SpellId, &'char Spell)> {
        self.0.control_spell(circle)
    }
}