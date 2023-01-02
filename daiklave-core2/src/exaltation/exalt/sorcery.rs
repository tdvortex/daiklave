use crate::{
    exaltation::exalt::exalt_type::solar::SolarSorcererView,
    sorcery::{
        ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId, SorceryCircle, Spell,
        SpellId,
    },
};

pub(crate) enum ExaltSorcery<'view, 'source> {
    Solar(&'view SolarSorcererView<'source>),
}

impl<'view, 'source> ExaltSorcery<'view, 'source> {
    pub fn archetype(&self, id: SorceryArchetypeId) -> Option<&'source SorceryArchetype> {
        match self {
            ExaltSorcery::Solar(solar_sorcerer) => solar_sorcerer.archetype(id),
        }
    }

    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> Option<(ShapingRitualId, &'source ShapingRitual)> {
        match self {
            ExaltSorcery::Solar(solar_sorcerer) => solar_sorcerer.shaping_ritual(circle),
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> Option<(SpellId, &'source Spell)> {
        match self {
            ExaltSorcery::Solar(solar_sorcerer) => solar_sorcerer.control_spell(circle),
        }
    }
}
