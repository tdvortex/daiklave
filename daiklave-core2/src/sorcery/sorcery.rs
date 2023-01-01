use super::{sorcery_switch::SorcerySwitch, archetype_id::SorceryArchetypeId, archetype::SorceryArchetype, sorcery_circle::SorceryCircle, shaping_ritual_id::ShapingRitualId, shaping_ritual::ShapingRitual, spell_id::SpellId, spell::Spell};

/// A character's Sorcery abilities.
pub struct Sorcery<'char>(pub(crate) SorcerySwitch<'char>);

impl<'char> Sorcery<'char> {
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