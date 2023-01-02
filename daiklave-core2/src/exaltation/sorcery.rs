use crate::{
    exaltation::exalt::ExaltSorcery,
    sorcery::{
        circles::terrestrial::sorcerer::TerrestrialCircleSorcerer, ShapingRitual, ShapingRitualId,
        SorceryArchetype, SorceryArchetypeId, SorceryCircle, Spell, SpellId,
    },
};

pub(crate) enum ExaltationSorcery<'view, 'source> {
    Mortal(&'view TerrestrialCircleSorcerer<'source>),
    Exalt(ExaltSorcery<'view, 'source>),
}

impl<'view, 'source> ExaltationSorcery<'view, 'source> {
    pub fn archetype(&self, id: SorceryArchetypeId) -> Option<&'source SorceryArchetype> {
        match self {
            ExaltationSorcery::Mortal(terrestrial) => terrestrial.archetype(id),
            ExaltationSorcery::Exalt(exalt_switch) => exalt_switch.archetype(id),
        }
    }

    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> Option<(ShapingRitualId, &'source ShapingRitual)> {
        match (self, circle) {
            (ExaltationSorcery::Mortal(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.shaping_ritual())
            }
            (ExaltationSorcery::Mortal(_), _) => None,
            (ExaltationSorcery::Exalt(exalt_switch), circle) => exalt_switch.shaping_ritual(circle),
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> Option<(SpellId, &'source Spell)> {
        match (self, circle) {
            (ExaltationSorcery::Mortal(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.control_spell())
            }
            (ExaltationSorcery::Mortal(_), _) => None,
            (ExaltationSorcery::Exalt(exalt_switch), circle) => exalt_switch.control_spell(circle),
        }
    }
}