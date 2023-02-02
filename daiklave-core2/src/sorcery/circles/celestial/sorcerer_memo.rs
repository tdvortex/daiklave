use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    merits::merit::{SorceryArchetypeMeritDetails, SorceryArchetypeMeritName},
    sorcery::{
        archetype::SorceryArchetypeName, circles::terrestrial::TerrestrialSpell, spell::SpellName,
        ShapingRitualDetails, SorceryArchetypeDetails,
    },
};

use super::spell::CelestialSpell;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CelestialCircleSorcererMemo {
    pub(in crate::sorcery::circles) archetypes: HashMap<
        SorceryArchetypeName,
        (
            SorceryArchetypeDetails,
            HashMap<SorceryArchetypeMeritName, SorceryArchetypeMeritDetails>,
        ),
    >,
    pub(in crate::sorcery::circles) circle_archetypes: [SorceryArchetypeName; 2],
    pub(in crate::sorcery::circles) shaping_ritual_names: [String; 2],
    pub(in crate::sorcery::circles) shaping_rituals: [ShapingRitualDetails; 2],
    pub(in crate::sorcery::circles) terrestrial_control_spell_name: SpellName,
    pub(in crate::sorcery::circles) terrestrial_control_spell: TerrestrialSpell,
    pub(in crate::sorcery::circles) terrestrial_spells: HashMap<SpellName, TerrestrialSpell>,
    pub(in crate::sorcery::circles) celestial_control_spell_name: SpellName,
    pub(in crate::sorcery::circles) celestial_control_spell: CelestialSpell,
    pub(in crate::sorcery::circles) celestial_spells: HashMap<SpellName, CelestialSpell>,
}
