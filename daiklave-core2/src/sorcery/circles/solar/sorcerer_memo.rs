use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{sorcery::{
    circles::{celestial::CelestialSpell, terrestrial::TerrestrialSpell},
    spell::SpellName,
    ShapingRitualDetails, SorceryArchetypeDetails,
    SorceryArchetypeName,
}, merits::merit::{SorceryArchetypeMeritDetails, SorceryArchetypeMeritName}};

use super::{SolarSpell};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SolarCircleSorcererMemo {
    pub(in crate::sorcery::circles) archetypes: HashMap<
        SorceryArchetypeName,
        (
            SorceryArchetypeDetails,
            HashMap<SorceryArchetypeMeritName, SorceryArchetypeMeritDetails>,
        ),
    >,
    pub(in crate::sorcery::circles) circle_archetypes: [SorceryArchetypeName; 3],
    pub(in crate::sorcery::circles) shaping_ritual_names: [String; 3],
    pub(in crate::sorcery::circles) shaping_rituals: [ShapingRitualDetails; 3],
    pub(in crate::sorcery::circles) terrestrial_control_spell_name: SpellName,
    pub(in crate::sorcery::circles) terrestrial_control_spell: TerrestrialSpell,
    pub(in crate::sorcery::circles) terrestrial_spells: HashMap<SpellName, TerrestrialSpell>,
    pub(in crate::sorcery::circles) celestial_control_spell_name: SpellName,
    pub(in crate::sorcery::circles) celestial_control_spell: CelestialSpell,
    pub(in crate::sorcery::circles) celestial_spells: HashMap<SpellName, CelestialSpell>,
    pub(in crate::sorcery::circles) solar_control_spell_name: SpellName,
    pub(in crate::sorcery::circles) solar_control_spell: SolarSpell,
    pub(in crate::sorcery::circles) solar_spells: HashMap<SpellName, SolarSpell>,
}