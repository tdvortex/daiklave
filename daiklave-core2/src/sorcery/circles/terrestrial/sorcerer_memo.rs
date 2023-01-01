use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::sorcery::{SorceryArchetypeId, SorceryArchetype, ShapingRitualId, ShapingRitual, SpellId, SorceryError, Spell, circles::{celestial::sorcerer_memo::CelestialCircleSorcererMemo, solar::sorcerer_memo::SolarCircleSorcererMemo}};

use super::{TerrestrialSpell, sorcerer_view::TerrestrialCircleSorcererView};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TerrestrialCircleSorcererMemo {
    archetype_id: SorceryArchetypeId,
    archetype: SorceryArchetype,
    shaping_ritual_id: ShapingRitualId,
    shaping_ritual: ShapingRitual,
    control_spell_id: SpellId,
    control_spell: TerrestrialSpell,
    other_spells: HashMap<SpellId, TerrestrialSpell>,
}

impl TerrestrialCircleSorcererMemo {
    pub fn _new(
        archetype_id: SorceryArchetypeId,
        archetype: SorceryArchetype,
        shaping_ritual_id: ShapingRitualId,
        shaping_ritual: ShapingRitual,
        control_spell_id: SpellId,
        control_spell: TerrestrialSpell,
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

    pub fn archetype(&self, id: SorceryArchetypeId) -> Option<&SorceryArchetype> {
        if id == self.archetype_id {
            Some(&self.archetype)
        } else {
            None
        }
    }

    pub fn shaping_ritual(&self) -> (ShapingRitualId, &ShapingRitual) {
        (self.shaping_ritual_id, &self.shaping_ritual)
    }

    pub fn control_spell(&self) -> (SpellId, &Spell) {
        (self.control_spell_id, &self.control_spell)
    }
}

impl<'char> TerrestrialCircleSorcererMemo {
    pub(crate) fn as_view(&'char self) -> TerrestrialCircleSorcererView<'char> {
        TerrestrialCircleSorcererView {
            archetype_id: self.archetype_id,
            archetype: &self.archetype,
            shaping_ritual_id: self.shaping_ritual_id,
            shaping_ritual: &self.shaping_ritual,
            control_spell_id: self.control_spell_id,
            control_spell: &self.control_spell,
            other_spells: self.other_spells.iter().map(|(k, v)| (*k, v)).collect(),
        }
    }
}

impl From<CelestialCircleSorcererMemo> for TerrestrialCircleSorcererMemo {
    fn from(mut celestial: CelestialCircleSorcererMemo) -> Self {
        Self {
            archetype_id: celestial.circle_archetypes[0],
            archetype: celestial
                .archetypes
                .remove(&celestial.circle_archetypes[0])
                .expect("Archetypes should be owned"),
            shaping_ritual_id: celestial.shaping_ritual_ids[0],
            shaping_ritual: celestial.shaping_rituals[0].clone(),
            control_spell_id: celestial.terrestrial_control_spell_id,
            control_spell: celestial.terrestrial_control_spell,
            other_spells: celestial.terrestrial_spells,
        }
    }
}

impl From<SolarCircleSorcererMemo> for TerrestrialCircleSorcererMemo {
    fn from(mut solar: SolarCircleSorcererMemo) -> Self {
        Self {
            archetype_id: solar.circle_archetypes[0],
            archetype: solar
                .archetypes
                .remove(&solar.circle_archetypes[0])
                .expect("Archetypes should be owned"),
            shaping_ritual_id: solar.shaping_ritual_ids[0],
            shaping_ritual: solar.shaping_rituals[0].clone(),
            control_spell_id: solar.terrestrial_control_spell_id,
            control_spell: solar.terrestrial_control_spell,
            other_spells: solar.terrestrial_spells,
        }
    }
}

impl<'source> From<TerrestrialCircleSorcererView<'source>> for TerrestrialCircleSorcererMemo {
    fn from(view: TerrestrialCircleSorcererView) -> Self {
        Self {
            archetype_id: view.archetype_id,
            archetype: view.archetype.to_owned(),
            shaping_ritual_id: view.shaping_ritual_id,
            shaping_ritual: view.shaping_ritual.to_owned(),
            control_spell_id: view.control_spell_id,
            control_spell: view.control_spell.to_owned(),
            other_spells: view
                .other_spells
                .into_iter()
                .map(|(k, v)| (k, v.to_owned()))
                .collect(),
        }
    }
}