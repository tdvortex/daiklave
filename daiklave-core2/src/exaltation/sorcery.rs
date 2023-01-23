use crate::{
    exaltation::exalt::ExaltSorcery,
    sorcery::{
        circles::terrestrial::sorcerer::TerrestrialCircleSorcerer,
        spell::{Spell, SpellId},
        ShapingRitual, ShapingRitualId, SorceryArchetypeId, SorceryArchetypeWithMerits,
        SorceryCircle,
    },
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum ExaltationSorcery<'view, 'source> {
    Mortal(&'view TerrestrialCircleSorcerer<'source>),
    Exalt(ExaltSorcery<'view, 'source>),
}

impl<'view, 'source> ExaltationSorcery<'view, 'source> {
    pub fn archetype(
        &self,
        id: SorceryArchetypeId,
    ) -> Option<SorceryArchetypeWithMerits<'view, 'source>> {
        match self {
            ExaltationSorcery::Mortal(terrestrial) => (*terrestrial).archetype(id),
            ExaltationSorcery::Exalt(exalt_switch) => exalt_switch.archetype(id),
        }
    }

    pub fn archetypes_iter(&self) -> impl Iterator<Item = SorceryArchetypeId> + '_ {
        match self {
            ExaltationSorcery::Mortal(terrestrial) => {
                std::iter::once(terrestrial.archetype_id).collect::<Vec<SorceryArchetypeId>>()
            }
            ExaltationSorcery::Exalt(exalt) => {
                exalt.archetypes_iter().collect::<Vec<SorceryArchetypeId>>()
            }
        }
        .into_iter()
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

    pub fn control_spell(&self, circle: SorceryCircle) -> Option<(SpellId, Spell<'source>)> {
        match (self, circle) {
            (ExaltationSorcery::Mortal(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.control_spell())
            }
            (ExaltationSorcery::Mortal(_), _) => None,
            (ExaltationSorcery::Exalt(exalt_switch), circle) => exalt_switch.control_spell(circle),
        }
    }

    pub fn get_spell(&self, spell_id: SpellId) -> Option<(Spell<'source>, bool)> {
        match self {
            ExaltationSorcery::Mortal(terrestrial) => terrestrial.get_spell(spell_id),
            ExaltationSorcery::Exalt(exalt_switch) => exalt_switch.get_spell(spell_id),
        }
    }

    pub fn iter_spells(&self) -> impl Iterator<Item = SpellId> + '_ {
        match self {
            ExaltationSorcery::Mortal(terrestrial) => {
                terrestrial.spells_iter().collect::<Vec<SpellId>>()
            }
            ExaltationSorcery::Exalt(exalt_switch) => {
                exalt_switch.spells_iter().collect::<Vec<SpellId>>()
            }
        }
        .into_iter()
    }
}
