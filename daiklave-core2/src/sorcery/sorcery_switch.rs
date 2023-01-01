use super::{terrestrial_circle_sorcerer::TerrestrialCircleSorcerer, archetype_id::SorceryArchetypeId, archetype::SorceryArchetype, sorcery_circle::SorceryCircle, shaping_ritual_id::ShapingRitualId, shaping_ritual::ShapingRitual, spell_id::SpellId, spell::Spell, exalt_sorcery_switch::ExaltSorcerySwitch};

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