use crate::{exaltation::exalt::exalt_type::solar::SolarSorcererMemo, sorcery::{SorceryArchetypeId, SorceryArchetype, SorceryCircle, ShapingRitualId, ShapingRitual, SpellId, Spell}};

pub(crate) enum ExaltSorcerySwitchMemo<'char> {
    Solar(&'char SolarSorcererMemo),
}

impl<'char> ExaltSorcerySwitchMemo<'char> {
    pub fn archetype(&self, id: SorceryArchetypeId) -> Option<&SorceryArchetype> {
        match self {
            ExaltSorcerySwitchMemo::Solar(solar_sorcerer) => solar_sorcerer.archetype(id),
        }
    }

    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> Option<(ShapingRitualId, &ShapingRitual)> {
        match self {
            ExaltSorcerySwitchMemo::Solar(solar_sorcerer) => solar_sorcerer.shaping_ritual(circle),
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> Option<(SpellId, &Spell)> {
        match self {
            ExaltSorcerySwitchMemo::Solar(solar_sorcerer) => solar_sorcerer.control_spell(circle),
        }
    }
}