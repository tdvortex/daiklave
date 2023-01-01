use super::{solar_sorcerer_view::SolarSorcererView, archetype_id::SorceryArchetypeId, archetype::SorceryArchetype, sorcery_circle::SorceryCircle, shaping_ritual_id::ShapingRitualId, shaping_ritual::ShapingRitual, spell_id::SpellId, spell::Spell};

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