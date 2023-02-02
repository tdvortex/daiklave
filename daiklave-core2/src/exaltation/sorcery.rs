use crate::{
    exaltation::exalt::ExaltSorcery,
    sorcery::{
        circles::terrestrial::sorcerer::TerrestrialCircleSorcerer, spell::Spell,
        SorceryArchetype, SorceryCircle, ShapingRitual,
    },
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum ExaltationSorcery<'view, 'source> {
    Mortal(&'view TerrestrialCircleSorcerer<'source>),
    Exalt(ExaltSorcery<'view, 'source>),
}

impl<'view, 'source> ExaltationSorcery<'view, 'source> {
    pub fn archetype(&self, name: &str) -> Option<SorceryArchetype<'view, 'source>> {
        match self {
            ExaltationSorcery::Mortal(terrestrial) => (*terrestrial).archetype(name),
            ExaltationSorcery::Exalt(exalt_switch) => exalt_switch.archetype(name),
        }
    }

    pub fn archetypes_iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        match self {
            ExaltationSorcery::Mortal(terrestrial) => {
                std::iter::once(terrestrial.archetype_name).collect::<Vec<&str>>()
            }
            ExaltationSorcery::Exalt(exalt) => exalt.archetypes_iter().collect::<Vec<&str>>(),
        }
        .into_iter()
    }

    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> Option<ShapingRitual<'source>> {
        match (self, circle) {
            (ExaltationSorcery::Mortal(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.shaping_ritual())
            }
            (ExaltationSorcery::Mortal(_), _) => None,
            (ExaltationSorcery::Exalt(exalt_switch), circle) => exalt_switch.shaping_ritual(circle),
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> Option<Spell<'source>> {
        match (self, circle) {
            (ExaltationSorcery::Mortal(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.control_spell())
            }
            (ExaltationSorcery::Mortal(_), _) => None,
            (ExaltationSorcery::Exalt(exalt_switch), circle) => exalt_switch.control_spell(circle),
        }
    }

    pub fn get_spell(&self, name: &str) -> Option<(Spell<'source>, bool)> {
        match self {
            ExaltationSorcery::Mortal(terrestrial) => terrestrial.get_spell(name),
            ExaltationSorcery::Exalt(exalt_switch) => exalt_switch.get_spell(name),
        }
    }

    pub fn iter_spells(&self) -> impl Iterator<Item = &'source str> + '_ {
        match self {
            ExaltationSorcery::Mortal(terrestrial) => {
                terrestrial.spells_iter().collect::<Vec<&str>>()
            }
            ExaltationSorcery::Exalt(exalt_switch) => {
                exalt_switch.spells_iter().collect::<Vec<&str>>()
            }
        }
        .into_iter()
    }
}
