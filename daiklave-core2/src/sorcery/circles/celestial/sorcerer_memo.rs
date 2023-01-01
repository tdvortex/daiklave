use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::sorcery::{SorceryArchetypeId, SorceryArchetype, ShapingRitualId, ShapingRitual, SpellId, circles::{terrestrial::TerrestrialSpell}};

use super::{spell::CelestialSpell, sorcerer_view::CelestialCircleSorcererView};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CelestialCircleSorcererMemo {
    archetypes: HashMap<SorceryArchetypeId, SorceryArchetype>,
    circle_archetypes: [SorceryArchetypeId; 2],
    shaping_ritual_ids: [ShapingRitualId; 2],
    shaping_rituals: [ShapingRitual; 2],
    terrestrial_control_spell_id: SpellId,
    terrestrial_control_spell: TerrestrialSpell,
    terrestrial_spells: HashMap<SpellId, TerrestrialSpell>,
    celestial_control_spell_id: SpellId,
    celestial_control_spell: CelestialSpell,
    celestial_spells: HashMap<SpellId, CelestialSpell>,
}

impl<'source> CelestialCircleSorcererMemo {
    pub fn as_ref(&'source self) -> CelestialCircleSorcererView<'source> {
        CelestialCircleSorcererView {
            archetypes: self.archetypes.iter().map(|(k, v)| (*k, v)).collect(),
            circle_archetypes: self.circle_archetypes,
            shaping_ritual_ids: self.shaping_ritual_ids,
            shaping_rituals: {
                self.shaping_rituals.iter().enumerate().fold([None; 2], |mut opt_arr, (i, el)| {
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
        }
    }
}