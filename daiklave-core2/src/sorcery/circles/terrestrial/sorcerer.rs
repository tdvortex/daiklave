use std::collections::HashMap;

use crate::sorcery::{
    circles::{celestial::sorcerer::CelestialCircleSorcerer, solar::sorcerer::SolarCircleSorcerer},
    ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId, SorceryError, Spell,
    SpellId, SorceryArchetypeMeritId, SorceryArchetypeWithMerits, SorceryArchetypeMerit,
};

use super::{sorcerer_memo::TerrestrialCircleSorcererMemo, TerrestrialSpell};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerrestrialCircleSorcerer<'source> {
    pub archetype_id: SorceryArchetypeId,
    pub archetype: &'source SorceryArchetype,
    pub archetype_merits: HashMap<SorceryArchetypeMeritId, &'source SorceryArchetypeMerit>,
    pub shaping_ritual_id: ShapingRitualId,
    pub shaping_ritual: &'source ShapingRitual,
    pub control_spell_id: SpellId,
    pub control_spell: &'source TerrestrialSpell,
    pub other_spells: HashMap<SpellId, &'source TerrestrialSpell>,
}

impl<'view, 'source> TerrestrialCircleSorcerer<'source> {
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
            archetype_merits: HashMap::new(),
            shaping_ritual_id,
            shaping_ritual,
            control_spell_id,
            control_spell,
            other_spells: HashMap::new(),
        })
    }

    pub fn as_memo(&self) -> TerrestrialCircleSorcererMemo {
        TerrestrialCircleSorcererMemo {
            archetype_id: self.archetype_id,
            archetype: self.archetype.to_owned(),
            archetype_merits: self.archetype_merits.iter().map(|(k, v)| (*k, (*v).to_owned())).collect(),
            shaping_ritual_id: self.shaping_ritual_id,
            shaping_ritual: self.shaping_ritual.to_owned(),
            control_spell_id: self.control_spell_id,
            control_spell: self.control_spell.to_owned(),
            other_spells: self
                .other_spells
                .iter()
                .map(|(k, v)| (*k, (*v).to_owned()))
                .collect(),
        }
    }

    pub fn archetype(&'view self, id: SorceryArchetypeId) -> Option<SorceryArchetypeWithMerits<'view, 'source>> {
        if id == self.archetype_id {
            Some((
                self.archetype,
                &self.archetype_merits
            ))
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

impl<'view, 'source> From<&'view CelestialCircleSorcerer<'source>>
    for TerrestrialCircleSorcerer<'source>
{
    fn from(celestial: &'view CelestialCircleSorcerer<'source>) -> Self {
        Self {
            archetype_id: celestial.circle_archetypes[0],
            archetype: celestial
                .archetypes
                .get(&celestial.circle_archetypes[0])
                .map(|(archetype, _merits)| *archetype)
                .expect("Archetypes should be owned"),
            archetype_merits: celestial
                .archetypes
                .get(&celestial.circle_archetypes[0])
                .map(|(_archetype, merits)| merits.to_owned())
                .expect("Archetypes should be owned"),
            shaping_ritual_id: celestial.shaping_ritual_ids[0],
            shaping_ritual: celestial.shaping_rituals[0],
            control_spell_id: celestial.terrestrial_control_spell_id,
            control_spell: celestial.terrestrial_control_spell,
            other_spells: celestial.terrestrial_spells.clone(),
        }
    }
}

impl<'view, 'source> From<&'view SolarCircleSorcerer<'source>>
    for TerrestrialCircleSorcerer<'source>
{
    fn from(solar: &'view SolarCircleSorcerer<'source>) -> Self {
        Self {
            archetype_id: solar.circle_archetypes[0],
            archetype: solar
                .archetypes
                .get(&solar.circle_archetypes[0])
                .map(|(archetype, _merits)| *archetype)
                .expect("Archetypes should be owned"),
            archetype_merits: solar
                .archetypes
                .get(&solar.circle_archetypes[0])
                .map(|(_archetype, merits)| merits.to_owned())
                .expect("Archetypes should be owned"),
            shaping_ritual_id: solar.shaping_ritual_ids[0],
            shaping_ritual: solar.shaping_rituals[0],
            control_spell_id: solar.terrestrial_control_spell_id,
            control_spell: solar.terrestrial_control_spell,
            other_spells: solar.terrestrial_spells.clone(),
        }
    }
}
