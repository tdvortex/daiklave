use crate::{
    exaltation::exalt::exalt_type::solar::SolarSorcererView,
    sorcery::{spell::Spell, SorceryArchetype, SorceryCircle, ShapingRitual},
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum ExaltSorcery<'view, 'source> {
    Solar(&'view SolarSorcererView<'source>),
}

impl<'view, 'source> ExaltSorcery<'view, 'source> {
    pub fn archetype(&self, name: &str) -> Option<SorceryArchetype<'view, 'source>> {
        match self {
            ExaltSorcery::Solar(solar_sorcerer) => solar_sorcerer.archetype(name),
        }
    }

    pub fn archetypes_iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        match self {
            ExaltSorcery::Solar(solar_sorcerer) => {
                solar_sorcerer.archetypes_iter().collect::<Vec<&str>>()
            }
        }
        .into_iter()
    }

    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> Option<ShapingRitual<'source>> {
        match self {
            ExaltSorcery::Solar(solar_sorcerer) => solar_sorcerer.shaping_ritual(circle),
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> Option<Spell<'source>> {
        match self {
            ExaltSorcery::Solar(solar_sorcerer) => solar_sorcerer.control_spell(circle),
        }
    }

    pub fn get_spell(&self, name: &str) -> Option<(Spell<'source>, bool)> {
        match self {
            ExaltSorcery::Solar(solar_sorcerer) => solar_sorcerer.get_spell(name),
        }
    }

    pub fn spells_iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        match self {
            ExaltSorcery::Solar(solar_sorcerer) => solar_sorcerer.spells_iter(),
        }
    }
}
