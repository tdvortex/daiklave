use std::collections::HashMap;

use crate::sorcery::{SorceryArchetypeId, SorceryArchetype, ShapingRitualId, ShapingRitual, SpellId, SorceryError, Spell, circles::{celestial::sorcerer_view::CelestialCircleSorcererView, solar::sorcerer_view::SolarCircleSorcererView}};

use super::TerrestrialSpell;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerrestrialCircleSorcererView<'source> {
    pub(crate) archetype_id: SorceryArchetypeId,
    pub(crate) archetype: &'source SorceryArchetype,
    pub(crate) shaping_ritual_id: ShapingRitualId,
    pub(crate) shaping_ritual: &'source ShapingRitual,
    pub(crate) control_spell_id: SpellId,
    pub(crate) control_spell: &'source TerrestrialSpell,
    pub(crate) other_spells: HashMap<SpellId, &'source TerrestrialSpell>,
}

impl<'source> TerrestrialCircleSorcererView<'source> {
    pub fn new(
        archetype_id: SorceryArchetypeId,
        archetype: &'source SorceryArchetype,
        shaping_ritual_id: ShapingRitualId,
        shaping_ritual: &'source ShapingRitual,
        control_spell_id: SpellId,
        control_spell: &'source TerrestrialSpell,
    ) -> Result<Self, SorceryError> {
        if shaping_ritual.archetype_id() != archetype_id {
            return Err(SorceryError::MissingArchetype);
        }

        Ok(Self {
            archetype_id,
            archetype,
            shaping_ritual_id,
            shaping_ritual,
            control_spell_id,
            control_spell,
            other_spells: HashMap::new(),
        })
    }
    pub fn archetype(&self, id: SorceryArchetypeId) -> Option<&'source SorceryArchetype> {
        if id == self.archetype_id {
            Some(self.archetype)
        } else {
            None
        }
    }

    pub fn shaping_ritual(&self) -> (ShapingRitualId, &'source ShapingRitual) {
        (self.shaping_ritual_id, self.shaping_ritual)
    }

    pub fn control_spell(&self) -> (SpellId, &'source Spell) {
        (self.control_spell_id, self.control_spell)
    }
}

impl<'view, 'source> From<&'view CelestialCircleSorcererView<'source>>
    for TerrestrialCircleSorcererView<'source>
{
    fn from(celestial: &'view CelestialCircleSorcererView<'source>) -> Self {
        Self {
            archetype_id: celestial.circle_archetypes[0],
            archetype: celestial
                .archetypes
                .get(&celestial.circle_archetypes[0])
                .expect("Archetypes should be owned"),
            shaping_ritual_id: celestial.shaping_ritual_ids[0],
            shaping_ritual: celestial.shaping_rituals[0],
            control_spell_id: celestial.terrestrial_control_spell_id,
            control_spell: celestial.terrestrial_control_spell,
            other_spells: celestial.terrestrial_spells.clone(),
        }
    }
}

impl<'view, 'source> From<&'view SolarCircleSorcererView<'source>>
    for TerrestrialCircleSorcererView<'source>
{
    fn from(solar: &'view SolarCircleSorcererView<'source>) -> Self {
        Self {
            archetype_id: solar.circle_archetypes[0],
            archetype: solar
                .archetypes
                .get(&solar.circle_archetypes[0])
                .expect("Archetypes should be owned"),
            shaping_ritual_id: solar.shaping_ritual_ids[0],
            shaping_ritual: solar.shaping_rituals[0],
            control_spell_id: solar.terrestrial_control_spell_id,
            control_spell: solar.terrestrial_control_spell,
            other_spells: solar.terrestrial_spells.clone(),
        }
    }
}