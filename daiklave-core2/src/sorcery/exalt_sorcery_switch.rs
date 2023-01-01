use super::{solar_sorcerer::SolarSorcerer, archetype_id::SorceryArchetypeId, archetype::SorceryArchetype, sorcery_circle::SorceryCircle, shaping_ritual_id::ShapingRitualId, shaping_ritual::ShapingRitual, spell_id::SpellId, spell::Spell};

pub(crate) enum ExaltSorcerySwitch<'char> {
    Solar(&'char SolarSorcerer),
}

impl<'char> ExaltSorcerySwitch<'char> {
    pub fn archetype(&self, id: SorceryArchetypeId) -> Option<&SorceryArchetype> {
        match self {
            ExaltSorcerySwitch::Solar(solar_sorcerer) => solar_sorcerer.archetype(id),
        }
    }

    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> Option<(ShapingRitualId, &ShapingRitual)> {
        match self {
            ExaltSorcerySwitch::Solar(solar_sorcerer) => solar_sorcerer.shaping_ritual(circle),
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> Option<(SpellId, &Spell)> {
        match self {
            ExaltSorcerySwitch::Solar(solar_sorcerer) => solar_sorcerer.control_spell(circle),
        }
    }
}