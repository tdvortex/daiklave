use serde::{Serialize, Deserialize};

use crate::sorcery::{circles::{terrestrial::sorcerer_memo::TerrestrialCircleSorcererMemo, celestial::sorcerer_memo::CelestialCircleSorcererMemo, solar::sorcerer_memo::SolarCircleSorcererMemo}, SorceryArchetypeId, SorceryArchetype, SorceryCircle, ShapingRitualId, ShapingRitual, SpellId, Spell};

use super::sorcery_view::SolarSorcererView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SolarSorcererMemo {
    Terrestrial(Box<TerrestrialCircleSorcererMemo>),
    Celestial(Box<CelestialCircleSorcererMemo>),
    Solar(Box<SolarCircleSorcererMemo>),
}

impl SolarSorcererMemo {
    pub fn archetype(&self, id: SorceryArchetypeId) -> Option<&SorceryArchetype> {
        match self {
            SolarSorcererMemo::Terrestrial(terrestrial) => terrestrial.archetype(id),
            SolarSorcererMemo::Celestial(celestial) => celestial.archetype(id),
            SolarSorcererMemo::Solar(solar) => solar.archetype(id),
        }
    }

    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> Option<(ShapingRitualId, &ShapingRitual)> {
        match (self, circle) {
            (SolarSorcererMemo::Terrestrial(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.shaping_ritual())
            }
            (SolarSorcererMemo::Terrestrial(_), _) => None,
            (SolarSorcererMemo::Celestial(celestial), circle) => celestial.shaping_ritual(circle),
            (SolarSorcererMemo::Solar(solar), circle) => Some(solar.shaping_ritual(circle)),
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> Option<(SpellId, &Spell)> {
        match (self, circle) {
            (SolarSorcererMemo::Terrestrial(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.control_spell())
            }
            (SolarSorcererMemo::Terrestrial(_), _) => None,
            (SolarSorcererMemo::Celestial(celestial), circle) => celestial.control_spell(circle),
            (SolarSorcererMemo::Solar(solar), circle) => Some(solar.control_spell(circle)),
        }
    }
}

impl<'char> SolarSorcererMemo {
    pub fn as_view(&'char self) -> SolarSorcererView<'char> {
        match self {
            SolarSorcererMemo::Terrestrial(terrestrial) => {
                SolarSorcererView::Terrestrial(terrestrial.as_view())
            }
            SolarSorcererMemo::Celestial(celestial) => {
                SolarSorcererView::Celestial(celestial.as_view())
            }
            SolarSorcererMemo::Solar(solar) => SolarSorcererView::Solar(solar.as_view()),
        }
    }
}

impl<'source> From<SolarSorcererView<'source>> for SolarSorcererMemo {
    fn from(view: SolarSorcererView) -> Self {
        match view {
            SolarSorcererView::Terrestrial(terrestrial) => {
                SolarSorcererMemo::Terrestrial(Box::new(terrestrial.into()))
            }
            SolarSorcererView::Celestial(celestial) => {
                SolarSorcererMemo::Celestial(Box::new(celestial.into()))
            }
            SolarSorcererView::Solar(solar) => SolarSorcererMemo::Solar(Box::new(solar.into())),
        }
    }
}