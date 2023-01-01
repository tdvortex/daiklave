use crate::sorcery::{circles::{terrestrial::sorcerer_view::TerrestrialCircleSorcererView, celestial::sorcerer_view::CelestialCircleSorcererView, solar::sorcerer_view::SolarCircleSorcererView}, SorceryArchetypeId, SorceryArchetype, SorceryCircle, ShapingRitualId, ShapingRitual, SpellId, Spell};

use super::SolarSorcererMemo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SolarSorcererView<'source> {
    Terrestrial(TerrestrialCircleSorcererView<'source>),
    Celestial(CelestialCircleSorcererView<'source>),
    Solar(SolarCircleSorcererView<'source>),
}

impl<'source> SolarSorcererView<'source> {
    pub fn as_memo(&self) -> SolarSorcererMemo {
        match self {
            SolarSorcererView::Terrestrial(view) => SolarSorcererMemo::Terrestrial(Box::new(view.as_memo())),
            SolarSorcererView::Celestial(view) => SolarSorcererMemo::Celestial(Box::new(view.as_memo())),
            SolarSorcererView::Solar(view) => SolarSorcererMemo::Solar(Box::new(view.as_memo())),
        }
    }

    pub fn archetype(&self, id: SorceryArchetypeId) -> Option<&'source SorceryArchetype> {
        match self {
            SolarSorcererView::Terrestrial(terrestrial) => terrestrial.archetype(id),
            SolarSorcererView::Celestial(celestial) => celestial.archetype(id),
            SolarSorcererView::Solar(solar) => solar.archetype(id),
        }
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

    pub fn control_spell(&self, circle: SorceryCircle) -> Option<(SpellId, &'source Spell)> {
        match (self, circle) {
            (SolarSorcererView::Terrestrial(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.control_spell())
            }
            (SolarSorcererView::Terrestrial(_), _) => None,
            (SolarSorcererView::Celestial(celestial), circle) => celestial.control_spell(circle),
            (SolarSorcererView::Solar(solar), circle) => Some(solar.control_spell(circle)),
        }
    }
}