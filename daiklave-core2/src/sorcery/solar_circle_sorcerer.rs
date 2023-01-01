use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use super::{archetype_id::SorceryArchetypeId, archetype::SorceryArchetype, shaping_ritual_id::ShapingRitualId, shaping_ritual::ShapingRitual, spell_id::SpellId, terrestrial_spell::TerrestrialSpell, celestial_spell::CelestialSpell, solar_spell::SolarSpell, sorcery_circle::SorceryCircle, spell::Spell, solar_circle_sorcerer_view::SolarCircleSorcererView};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SolarCircleSorcerer {
    pub(crate) archetypes: HashMap<SorceryArchetypeId, SorceryArchetype>,
    pub(crate) circle_archetypes: [SorceryArchetypeId; 3],
    pub(crate) shaping_ritual_ids: [ShapingRitualId; 3],
    pub(crate) shaping_rituals: [ShapingRitual; 3],
    pub(crate) terrestrial_control_spell_id: SpellId,
    pub(crate) terrestrial_control_spell: TerrestrialSpell,
    pub(crate) terrestrial_spells: HashMap<SpellId, TerrestrialSpell>,
    pub(crate) celestial_control_spell_id: SpellId,
    pub(crate) celestial_control_spell: CelestialSpell,
    pub(crate) celestial_spells: HashMap<SpellId, CelestialSpell>,
    pub(crate) solar_control_spell_id: SpellId,
    pub(crate) solar_control_spell: SolarSpell,
    pub(crate) solar_spells: HashMap<SpellId, SolarSpell>,
}

impl SolarCircleSorcerer {
    pub fn archetype(&self, id: SorceryArchetypeId) -> Option<&SorceryArchetype> {
        if self.circle_archetypes.contains(&id) {
            self.archetypes.get(&id)
        } else {
            None
        }
    }

    pub fn shaping_ritual(&self, circle: SorceryCircle) -> (ShapingRitualId, &ShapingRitual) {
        match circle {
            SorceryCircle::Terrestrial => (self.shaping_ritual_ids[0], &self.shaping_rituals[0]),
            SorceryCircle::Celestial => (self.shaping_ritual_ids[1], &self.shaping_rituals[1]),
            SorceryCircle::Solar => (self.shaping_ritual_ids[2], &self.shaping_rituals[2]),
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> (SpellId, &Spell) {
        match circle {
            SorceryCircle::Terrestrial => (
                self.terrestrial_control_spell_id,
                &self.terrestrial_control_spell,
            ),
            SorceryCircle::Celestial => (
                self.celestial_control_spell_id,
                &self.celestial_control_spell,
            ),
            SorceryCircle::Solar => (self.solar_control_spell_id, &self.solar_control_spell),
        }
    }
}

impl<'char> SolarCircleSorcerer {
    pub(crate) fn as_view(&'char self) -> SolarCircleSorcererView<'char> {
        SolarCircleSorcererView {
            archetypes: self.archetypes.iter().map(|(k, v)| (*k, v)).collect(),
            circle_archetypes: self.circle_archetypes,
            shaping_ritual_ids: self.shaping_ritual_ids,
            shaping_rituals: self
                .shaping_rituals
                .iter()
                .enumerate()
                .fold([None; 3], |mut opt_arr, (i, el)| {
                    opt_arr[i] = Some(el);
                    opt_arr
                })
                .map(|opt| opt.unwrap()),
            terrestrial_control_spell_id: self.terrestrial_control_spell_id,
            terrestrial_control_spell: &self.terrestrial_control_spell,
            terrestrial_spells: self
                .terrestrial_spells
                .iter()
                .map(|(k, v)| (*k, v))
                .collect(),
            celestial_control_spell_id: self.celestial_control_spell_id,
            celestial_control_spell: &self.celestial_control_spell,
            celestial_spells: self.celestial_spells.iter().map(|(k, v)| (*k, v)).collect(),
            solar_control_spell_id: self.solar_control_spell_id,
            solar_control_spell: &self.solar_control_spell,
            solar_spells: self.solar_spells.iter().map(|(k, v)| (*k, v)).collect(),
        }
    }
}

impl<'source> From<SolarCircleSorcererView<'source>> for SolarCircleSorcerer {
    fn from(view: SolarCircleSorcererView) -> Self {
        Self {
            archetypes: view
                .archetypes
                .into_iter()
                .map(|(k, v)| (k, v.to_owned()))
                .collect(),
            circle_archetypes: view.circle_archetypes,
            shaping_ritual_ids: view.shaping_ritual_ids,
            shaping_rituals: view.shaping_rituals.map(|ptr| ptr.to_owned()),
            terrestrial_control_spell_id: view.terrestrial_control_spell_id,
            terrestrial_control_spell: view.terrestrial_control_spell.to_owned(),
            terrestrial_spells: view
                .terrestrial_spells
                .into_iter()
                .map(|(k, v)| (k, v.to_owned()))
                .collect(),
            celestial_control_spell_id: view.celestial_control_spell_id,
            celestial_control_spell: view.celestial_control_spell.to_owned(),
            celestial_spells: view
                .celestial_spells
                .into_iter()
                .map(|(k, v)| (k, v.to_owned()))
                .collect(),
            solar_control_spell_id: view.solar_control_spell_id,
            solar_control_spell: view.solar_control_spell.to_owned(),
            solar_spells: view
                .solar_spells
                .into_iter()
                .map(|(k, v)| (k, v.to_owned()))
                .collect(),
        }
    }
}