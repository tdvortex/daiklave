mod sorcery_memo;

pub(crate) use sorcery_memo::SolarSorcererMemo;

use crate::sorcery::{
    circles::{
        celestial::sorcerer::CelestialCircleSorcerer, solar::sorcerer::SolarCircleSorcerer,
        terrestrial::sorcerer::TerrestrialCircleSorcerer,
    },
    spell::Spell,
    SorceryArchetype, SorceryCircle, ShapingRitual,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SolarSorcererView<'source> {
    Terrestrial(TerrestrialCircleSorcerer<'source>),
    Celestial(CelestialCircleSorcerer<'source>),
    Solar(SolarCircleSorcerer<'source>),
}

impl<'source> From<&'source SolarSorcererMemo> for SolarSorcererView<'source> {
    fn from(memo: &'source SolarSorcererMemo) -> Self {
        match memo {
            SolarSorcererMemo::Terrestrial(box_terrestrial) => Self::Terrestrial(box_terrestrial.as_ref().into()),
            SolarSorcererMemo::Celestial(box_celestial) => Self::Celestial(box_celestial.as_ref().into()),
            SolarSorcererMemo::Solar(box_solar) => Self::Solar(box_solar.as_ref().into()),
        }
    }
}

impl<'view, 'source> SolarSorcererView<'source> {
    pub fn archetype(
        &'view self,
        name: &str,
    ) -> Option<SorceryArchetype<'view, 'source>> {
        match self {
            SolarSorcererView::Terrestrial(terrestrial) => terrestrial.archetype(name),
            SolarSorcererView::Celestial(celestial) => celestial.archetype(name),
            SolarSorcererView::Solar(solar) => solar.archetype(name),
        }
    }

    pub fn archetypes_iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        match self {
            SolarSorcererView::Terrestrial(terrestrial) => {
                std::iter::once(terrestrial.archetype_name).collect::<Vec<&str>>()
            }
            SolarSorcererView::Celestial(celestial) => {
                celestial.archetypes.keys().copied().collect::<Vec<&str>>()
            }
            SolarSorcererView::Solar(solar) => {
                solar.archetypes.keys().copied().collect::<Vec<&str>>()
            }
        }
        .into_iter()
    }

    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> Option<ShapingRitual<'source>> {
        match (self, circle) {
            (SolarSorcererView::Terrestrial(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.shaping_ritual())
            }
            (SolarSorcererView::Terrestrial(_), _) => None,
            (SolarSorcererView::Celestial(celestial), circle) => celestial.shaping_ritual(circle),
            (SolarSorcererView::Solar(solar), circle) => Some(solar.shaping_ritual(circle)),
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> Option<Spell<'source>> {
        match (self, circle) {
            (SolarSorcererView::Terrestrial(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.control_spell())
            }
            (SolarSorcererView::Terrestrial(_), _) => None,
            (SolarSorcererView::Celestial(celestial), circle) => celestial.control_spell(circle),
            (SolarSorcererView::Solar(solar), circle) => Some(solar.control_spell(circle)),
        }
    }

    pub fn get_spell(&self, name: &str) -> Option<(Spell<'source>, bool)> {
        match self {
            SolarSorcererView::Terrestrial(terrestrial) => terrestrial.get_spell(name),
            SolarSorcererView::Celestial(celestial) => celestial.get_spell(name),
            SolarSorcererView::Solar(solar) => solar.get_spell(name),
        }
    }

    pub fn spells_iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        match self {
            SolarSorcererView::Terrestrial(terrestrial) => {
                terrestrial.spells_iter().collect::<Vec<&str>>()
            }
            SolarSorcererView::Celestial(celestial) => {
                celestial.spells_iter().collect::<Vec<&str>>()
            }
            SolarSorcererView::Solar(solar) => solar.spells_iter().collect::<Vec<&str>>(),
        }
        .into_iter()
    }
}
