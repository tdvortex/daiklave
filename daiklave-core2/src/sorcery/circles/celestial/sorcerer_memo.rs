use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::sorcery::{SorceryArchetypeId, SorceryArchetype, ShapingRitualId, ShapingRitual, SpellId, circles::{terrestrial::TerrestrialSpell, sorcery_circle::SorceryCircle}, Spell};

use super::{spell::CelestialSpell, sorcerer_view::CelestialCircleSorcererView};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CelestialCircleSorcererMemo {
    pub(crate) archetypes: HashMap<SorceryArchetypeId, SorceryArchetype>,
    pub(crate) circle_archetypes: [SorceryArchetypeId; 2],
    pub(crate) shaping_ritual_ids: [ShapingRitualId; 2],
    pub(crate) shaping_rituals: [ShapingRitual; 2],
    pub(crate) terrestrial_control_spell_id: SpellId,
    pub(crate) terrestrial_control_spell: TerrestrialSpell,
    pub(crate) terrestrial_spells: HashMap<SpellId, TerrestrialSpell>,
    pub(crate) celestial_control_spell_id: SpellId,
    pub(crate) celestial_control_spell: CelestialSpell,
    pub(crate) celestial_spells: HashMap<SpellId, CelestialSpell>,
}

impl CelestialCircleSorcererMemo {
    pub fn archetype(&self, id: SorceryArchetypeId) -> Option<&SorceryArchetype> {
        if self.circle_archetypes.contains(&id) {
            self.archetypes.get(&id)
        } else {
            None
        }
    }

    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> Option<(ShapingRitualId, &ShapingRitual)> {
        match circle {
            SorceryCircle::Terrestrial => {
                Some((self.shaping_ritual_ids[0], &self.shaping_rituals[0]))
            }
            SorceryCircle::Celestial => {
                Some((self.shaping_ritual_ids[1], &self.shaping_rituals[1]))
            }
            SorceryCircle::Solar => None,
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> Option<(SpellId, &Spell)> {
        match circle {
            SorceryCircle::Terrestrial => Some((
                self.terrestrial_control_spell_id,
                &self.terrestrial_control_spell,
            )),
            SorceryCircle::Celestial => Some((
                self.celestial_control_spell_id,
                &self.celestial_control_spell,
            )),
            SorceryCircle::Solar => None,
        }
    }
}

impl<'char> CelestialCircleSorcererMemo {
    pub(crate) fn as_view(&'char self) -> CelestialCircleSorcererView<'char> {
        CelestialCircleSorcererView {
            archetypes: self.archetypes.iter().map(|(k, v)| (*k, v)).collect(),
            circle_archetypes: self.circle_archetypes,
            shaping_ritual_ids: self.shaping_ritual_ids,
            shaping_rituals: self
                .shaping_rituals
                .iter()
                .enumerate()
                .fold([None; 2], |mut opt_arr, (i, el)| {
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
        }
    }
}

impl<'source> From<CelestialCircleSorcererView<'source>> for CelestialCircleSorcererMemo {
    fn from(view: CelestialCircleSorcererView) -> Self {
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
        }
    }
}