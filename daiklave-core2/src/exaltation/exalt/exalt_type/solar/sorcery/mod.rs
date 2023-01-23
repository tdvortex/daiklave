mod sorcery_memo;

pub(crate) use sorcery_memo::SolarSorcererMemo;

use crate::sorcery::{
    circles::{
        celestial::sorcerer::CelestialCircleSorcerer, solar::sorcerer::SolarCircleSorcerer,
        terrestrial::sorcerer::TerrestrialCircleSorcerer,
    },
    spell::{Spell, SpellId},
    ShapingRitual, ShapingRitualId, SorceryArchetypeId, SorceryArchetypeWithMerits, SorceryCircle,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SolarSorcererView<'source> {
    Terrestrial(TerrestrialCircleSorcerer<'source>),
    Celestial(CelestialCircleSorcerer<'source>),
    Solar(SolarCircleSorcerer<'source>),
}

impl<'view, 'source> SolarSorcererView<'source> {
    pub fn as_memo(&self) -> SolarSorcererMemo {
        match self {
            SolarSorcererView::Terrestrial(view) => {
                SolarSorcererMemo::Terrestrial(Box::new(view.as_memo()))
            }
            SolarSorcererView::Celestial(view) => {
                SolarSorcererMemo::Celestial(Box::new(view.as_memo()))
            }
            SolarSorcererView::Solar(view) => SolarSorcererMemo::Solar(Box::new(view.as_memo())),
        }
    }

    pub fn archetype(
        &'view self,
        id: SorceryArchetypeId,
    ) -> Option<SorceryArchetypeWithMerits<'view, 'source>> {
        match self {
            SolarSorcererView::Terrestrial(terrestrial) => terrestrial.archetype(id),
            SolarSorcererView::Celestial(celestial) => celestial.archetype(id),
            SolarSorcererView::Solar(solar) => solar.archetype(id),
        }
    }

    pub fn archetypes_iter(&self) -> impl Iterator<Item = SorceryArchetypeId> + '_ {
        match self {
            SolarSorcererView::Terrestrial(terrestrial) => {
                std::iter::once(terrestrial.archetype_id).collect::<Vec<SorceryArchetypeId>>()
            }
            SolarSorcererView::Celestial(celestial) => celestial
                .archetypes
                .keys()
                .copied()
                .collect::<Vec<SorceryArchetypeId>>(),
            SolarSorcererView::Solar(solar) => solar
                .archetypes
                .keys()
                .copied()
                .collect::<Vec<SorceryArchetypeId>>(),
        }
        .into_iter()
    }

    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> Option<(ShapingRitualId, &'source ShapingRitual)> {
        match (self, circle) {
            (SolarSorcererView::Terrestrial(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.shaping_ritual())
            }
            (SolarSorcererView::Terrestrial(_), _) => None,
            (SolarSorcererView::Celestial(celestial), circle) => celestial.shaping_ritual(circle),
            (SolarSorcererView::Solar(solar), circle) => Some(solar.shaping_ritual(circle)),
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> Option<(SpellId, Spell<'source>)> {
        match (self, circle) {
            (SolarSorcererView::Terrestrial(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.control_spell())
            }
            (SolarSorcererView::Terrestrial(_), _) => None,
            (SolarSorcererView::Celestial(celestial), circle) => celestial.control_spell(circle),
            (SolarSorcererView::Solar(solar), circle) => Some(solar.control_spell(circle)),
        }
    }

    pub fn get_spell(&self, spell_id: SpellId) -> Option<(Spell<'source>, bool)> {
        match self {
            SolarSorcererView::Terrestrial(terrestrial) => terrestrial.get_spell(spell_id),
            SolarSorcererView::Celestial(celestial) => celestial.get_spell(spell_id),
            SolarSorcererView::Solar(solar) => solar.get_spell(spell_id),
        }
    }

    pub fn spells_iter(&self) -> impl Iterator<Item = SpellId> + '_ {
        match self {
            SolarSorcererView::Terrestrial(terrestrial) => {
                terrestrial.spells_iter().collect::<Vec<SpellId>>()
            }
            SolarSorcererView::Celestial(celestial) => {
                celestial.spells_iter().collect::<Vec<SpellId>>()
            }
            SolarSorcererView::Solar(solar) => solar.spells_iter().collect::<Vec<SpellId>>(),
        }
        .into_iter()
    }
}
