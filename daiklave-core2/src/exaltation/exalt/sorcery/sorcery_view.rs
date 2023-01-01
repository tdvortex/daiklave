use crate::{
    exaltation::exalt::exalt_type::solar::SolarSorcererView,
    sorcery::{
        ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId, SorceryCircle, Spell,
        SpellId,
    },
};

pub(crate) enum ExaltSorceryViewSwitch<'view, 'source> {
    Solar(&'view SolarSorcererView<'source>),
}

impl<'view, 'source> ExaltSorceryViewSwitch<'view, 'source> {
    pub fn archetype(&self, id: SorceryArchetypeId) -> Option<&'source SorceryArchetype> {
        match self {
            ExaltSorceryViewSwitch::Solar(solar_sorcerer) => solar_sorcerer.archetype(id),
        }
    }

    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> Option<(ShapingRitualId, &'source ShapingRitual)> {
        match self {
            ExaltSorceryViewSwitch::Solar(solar_sorcerer) => solar_sorcerer.shaping_ritual(circle),
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> Option<(SpellId, &'source Spell)> {
        match self {
            ExaltSorceryViewSwitch::Solar(solar_sorcerer) => solar_sorcerer.control_spell(circle),
        }
    }
}