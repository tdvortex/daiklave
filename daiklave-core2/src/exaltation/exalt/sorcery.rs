use crate::{
    exaltation::exalt::exalt_type::solar::SolarSorcererView,
    sorcery::{
        spell::{Spell, SpellId},
        ShapingRitual, ShapingRitualId, SorceryArchetypeId, SorceryArchetypeWithMerits,
        SorceryCircle,
    },
};

pub(crate) enum ExaltSorcery<'view, 'source> {
    Solar(&'view SolarSorcererView<'source>),
}

impl<'view, 'source> ExaltSorcery<'view, 'source> {
    pub fn archetype(
        &self,
        id: SorceryArchetypeId,
    ) -> Option<SorceryArchetypeWithMerits<'view, 'source>> {
        match self {
            ExaltSorcery::Solar(solar_sorcerer) => solar_sorcerer.archetype(id),
        }
    }

    pub fn archetypes_iter(&self) -> impl Iterator<Item = SorceryArchetypeId> + '_ {
        match self {
            ExaltSorcery::Solar(solar_sorcerer) => solar_sorcerer
                .archetypes_iter()
                .collect::<Vec<SorceryArchetypeId>>(),
        }
        .into_iter()
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
