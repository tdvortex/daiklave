use super::{terrestrial_circle_sorcerer_view::TerrestrialCircleSorcererView, archetype_id::SorceryArchetypeId, archetype::SorceryArchetype, sorcery_circle::SorceryCircle, shaping_ritual_id::ShapingRitualId, shaping_ritual::ShapingRitual, spell_id::SpellId, spell::Spell, exalt_sorcery_view_switch::ExaltSorceryViewSwitch};

pub(crate) enum SorceryViewSwitch<'view, 'source> {
    Mortal(&'view TerrestrialCircleSorcererView<'source>),
    Exalt(ExaltSorceryViewSwitch<'view, 'source>),
}

impl<'view, 'source> SorceryViewSwitch<'view, 'source> {
    pub fn archetype(&self, id: SorceryArchetypeId) -> Option<&'source SorceryArchetype> {
        match self {
            SorceryViewSwitch::Mortal(terrestrial) => terrestrial.archetype(id),
            SorceryViewSwitch::Exalt(exalt_switch) => exalt_switch.archetype(id),
        }
    }

    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> Option<(ShapingRitualId, &'source ShapingRitual)> {
        match (self, circle) {
            (SorceryViewSwitch::Mortal(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.shaping_ritual())
            }
            (SorceryViewSwitch::Mortal(_), _) => None,
            (SorceryViewSwitch::Exalt(exalt_switch), circle) => exalt_switch.shaping_ritual(circle),
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> Option<(SpellId, &'source Spell)> {
        match (self, circle) {
            (SorceryViewSwitch::Mortal(terrestrial), SorceryCircle::Terrestrial) => {
                Some(terrestrial.control_spell())
            }
            (SorceryViewSwitch::Mortal(_), _) => None,
            (SorceryViewSwitch::Exalt(exalt_switch), circle) => exalt_switch.control_spell(circle),
        }
    }
}