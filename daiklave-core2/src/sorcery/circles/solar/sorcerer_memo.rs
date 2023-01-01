use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::sorcery::{SorceryArchetypeId, SorceryArchetype, ShapingRitualId, ShapingRitual, SpellId, circles::{terrestrial::TerrestrialSpell, celestial::CelestialSpell}};

use super::{SolarSpell};


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