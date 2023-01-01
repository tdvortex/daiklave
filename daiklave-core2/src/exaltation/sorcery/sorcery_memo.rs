use crate::{sorcery::{circles::terrestrial::sorcerer_memo::TerrestrialCircleSorcererMemo, SorceryArchetypeId, SorceryArchetype, SorceryCircle, ShapingRitualId, ShapingRitual, SpellId, Spell}, exaltation::exalt::ExaltSorcerySwitchMemo};

pub(crate) enum SorcerySwitchMemo<'char> {
    Mortal(&'char TerrestrialCircleSorcererMemo),
    Exalt(ExaltSorcerySwitchMemo<'char>),
}

impl<'char> SorcerySwitchMemo<'char> {
    pub fn archetype(&'char self, id: SorceryArchetypeId) -> Option<&'char SorceryArchetype> {
        match self {
            SorcerySwitchMemo::Mortal(terrestrial) => terrestrial.archetype(id),
            SorcerySwitchMemo::Exalt(exalt_switch) => exalt_switch.archetype(id),
        }
    }

    pub fn shaping_ritual(
        &'char self,
        circle: SorceryCircle,
    ) -> Option<(ShapingRitualId, &'char ShapingRitual)> {
        match (self, circle) {
            (SorcerySwitchMemo::Mortal(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.shaping_ritual())
            }
            (SorcerySwitchMemo::Mortal(_), _) => None,
            (SorcerySwitchMemo::Exalt(exalt_switch), circle) => exalt_switch.shaping_ritual(circle),
        }
    }

    pub fn control_spell(&'char self, circle: SorceryCircle) -> Option<(SpellId, &'char Spell)> {
        match (self, circle) {
            (SorcerySwitchMemo::Mortal(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.control_spell())
            }
            (SorcerySwitchMemo::Mortal(_), _) => None,
            (SorcerySwitchMemo::Exalt(exalt_switch), circle) => exalt_switch.control_spell(circle),
        }
    }
}