use crate::{sorcery::{circles::terrestrial::sorcerer::TerrestrialCircleSorcerer, SorceryArchetypeId, SorceryArchetype, SorceryCircle, ShapingRitualId, ShapingRitual, SpellId, Spell}, exaltation::exalt::ExaltSorcerySwitch};

pub(crate) enum SorcerySwitch<'char> {
    Mortal(&'char TerrestrialCircleSorcerer),
    Exalt(ExaltSorcerySwitch<'char>),
}

impl<'char> SorcerySwitch<'char> {
    pub fn archetype(&'char self, id: SorceryArchetypeId) -> Option<&'char SorceryArchetype> {
        match self {
            SorcerySwitch::Mortal(terrestrial) => terrestrial.archetype(id),
            SorcerySwitch::Exalt(exalt_switch) => exalt_switch.archetype(id),
        }
    }

    pub fn shaping_ritual(
        &'char self,
        circle: SorceryCircle,
    ) -> Option<(ShapingRitualId, &'char ShapingRitual)> {
        match (self, circle) {
            (SorcerySwitch::Mortal(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.shaping_ritual())
            }
            (SorcerySwitch::Mortal(_), _) => None,
            (SorcerySwitch::Exalt(exalt_switch), circle) => exalt_switch.shaping_ritual(circle),
        }
    }

    pub fn control_spell(&'char self, circle: SorceryCircle) -> Option<(SpellId, &'char Spell)> {
        match (self, circle) {
            (SorcerySwitch::Mortal(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.control_spell())
            }
            (SorcerySwitch::Mortal(_), _) => None,
            (SorcerySwitch::Exalt(exalt_switch), circle) => exalt_switch.control_spell(circle),
        }
    }
}