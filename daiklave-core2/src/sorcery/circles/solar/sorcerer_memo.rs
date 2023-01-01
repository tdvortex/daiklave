use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::sorcery::{SorceryArchetypeId, SorceryArchetype, ShapingRitualId, ShapingRitual, SpellId, circles::{terrestrial::TerrestrialSpell, celestial::CelestialSpell}};

use super::{SolarSpell, sorcerer_view::SolarCircleSorcererView};


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SolarCircleSorcererMemo {
    archetypes: HashMap<SorceryArchetypeId, SorceryArchetype>,
    circle_archetypes: [SorceryArchetypeId; 3],
    shaping_ritual_ids: [ShapingRitualId; 3],
    shaping_rituals: [ShapingRitual; 3],
    terrestrial_control_spell_id: SpellId,
    terrestrial_control_spell: TerrestrialSpell,
    terrestrial_spells: HashMap<SpellId, TerrestrialSpell>,
    celestial_control_spell_id: SpellId,
    celestial_control_spell: CelestialSpell,
    celestial_spells: HashMap<SpellId, CelestialSpell>,
    solar_control_spell_id: SpellId,
    solar_control_spell: SolarSpell,
    solar_spells: HashMap<SpellId, SolarSpell>,
}

impl<'source> SolarCircleSorcererMemo {
    pub fn as_ref(&'source self) -> SolarCircleSorcererView<'source> {
        SolarCircleSorcererView {
            archetypes: self.archetypes.iter().map(|(k, v)| (*k, v)).collect(),
            circle_archetypes: self.circle_archetypes,
            shaping_ritual_ids: self.shaping_ritual_ids,
            shaping_rituals: {
                self.shaping_rituals.iter().enumerate().fold([None; 3], |mut opt_arr, (i, el)| {
                    opt_arr[i] = Some(el);
                    opt_arr
                }).map(|opt| opt.unwrap())
            },
            terrestrial_control_spell_id: self.terrestrial_control_spell_id,
            terrestrial_control_spell: &self.terrestrial_control_spell,
            terrestrial_spells: self.terrestrial_spells.iter().map(|(k, v)| (*k, v)).collect(),
            celestial_control_spell_id: self.celestial_control_spell_id,
            celestial_control_spell: &self.celestial_control_spell,
            celestial_spells: self.celestial_spells.iter().map(|(k, v)| (*k, v)).collect(),
            solar_control_spell_id: self.solar_control_spell_id,
            solar_control_spell: &self.solar_control_spell,
            solar_spells: self.solar_spells.iter().map(|(k, v)| (*k, v)).collect(),
        }
    }
}