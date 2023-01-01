use serde::{Serialize, Deserialize};

use super::{terrestrial_circle_sorcerer::TerrestrialCircleSorcerer, celestial_circle_sorcerer::CelestialCircleSorcerer, solar_circle_sorcerer::SolarCircleSorcerer, archetype_id::SorceryArchetypeId, archetype::SorceryArchetype, sorcery_circle::SorceryCircle, shaping_ritual_id::ShapingRitualId, shaping_ritual::ShapingRitual, spell_id::SpellId, spell::Spell, solar_sorcerer_view::SolarSorcererView};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SolarSorcerer {
    Terrestrial(Box<TerrestrialCircleSorcerer>),
    Celestial(Box<CelestialCircleSorcerer>),
    Solar(Box<SolarCircleSorcerer>),
}

impl SolarSorcerer {
    pub fn archetype(&self, id: SorceryArchetypeId) -> Option<&SorceryArchetype> {
        match self {
            SolarSorcerer::Terrestrial(terrestrial) => terrestrial.archetype(id),
            SolarSorcerer::Celestial(celestial) => celestial.archetype(id),
            SolarSorcerer::Solar(solar) => solar.archetype(id),
        }
    }

    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> Option<(ShapingRitualId, &ShapingRitual)> {
        match (self, circle) {
            (SolarSorcerer::Terrestrial(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.shaping_ritual())
            }
            (SolarSorcerer::Terrestrial(_), _) => None,
            (SolarSorcerer::Celestial(celestial), circle) => celestial.shaping_ritual(circle),
            (SolarSorcerer::Solar(solar), circle) => Some(solar.shaping_ritual(circle)),
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> Option<(SpellId, &Spell)> {
        match (self, circle) {
            (SolarSorcerer::Terrestrial(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.control_spell())
            }
            (SolarSorcerer::Terrestrial(_), _) => None,
            (SolarSorcerer::Celestial(celestial), circle) => celestial.control_spell(circle),
            (SolarSorcerer::Solar(solar), circle) => Some(solar.control_spell(circle)),
        }
    }
}

impl<'char> SolarSorcerer {
    pub fn as_view(&'char self) -> SolarSorcererView<'char> {
        match self {
            SolarSorcerer::Terrestrial(terrestrial) => {
                SolarSorcererView::Terrestrial(terrestrial.as_view())
            }
            SolarSorcerer::Celestial(celestial) => {
                SolarSorcererView::Celestial(celestial.as_view())
            }
            SolarSorcerer::Solar(solar) => SolarSorcererView::Solar(solar.as_view()),
        }
    }
}

impl<'source> From<SolarSorcererView<'source>> for SolarSorcerer {
    fn from(view: SolarSorcererView) -> Self {
        match view {
            SolarSorcererView::Terrestrial(terrestrial) => {
                SolarSorcerer::Terrestrial(Box::new(terrestrial.into()))
            }
            SolarSorcererView::Celestial(celestial) => {
                SolarSorcerer::Celestial(Box::new(celestial.into()))
            }
            SolarSorcererView::Solar(solar) => SolarSorcerer::Solar(Box::new(solar.into())),
        }
    }
}