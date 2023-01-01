use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::sorcery::{SorceryArchetypeId, SorceryArchetype, ShapingRitualId, ShapingRitual, SpellId, circles::{terrestrial::TerrestrialSpell}};

use super::{spell::CelestialSpell};

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