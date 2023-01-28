use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::sorcery::{
    archetype::{ShapingRitualSummary, SorceryArchetypeName},
    circles::terrestrial::TerrestrialSpell,
    spell::SpellName,
    ShapingRitual, SorceryArchetype, SorceryArchetypeMerit,
};

use super::{sorcerer::CelestialCircleSorcerer, spell::CelestialSpell};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CelestialCircleSorcererMemo {
    pub(in crate::sorcery::circles) archetypes: HashMap<
        SorceryArchetypeName,
        (
            SorceryArchetype,
            HashMap<String, SorceryArchetypeMerit>,
        ),
    >,
    pub(in crate::sorcery::circles) circle_archetypes: [SorceryArchetypeName; 2],
    pub(in crate::sorcery::circles) shaping_ritual_names: [ShapingRitualSummary; 2],
    pub(in crate::sorcery::circles) shaping_rituals: [ShapingRitual; 2],
    pub(in crate::sorcery::circles) terrestrial_control_spell_name: SpellName,
    pub(in crate::sorcery::circles) terrestrial_control_spell: TerrestrialSpell,
    pub(in crate::sorcery::circles) terrestrial_spells: HashMap<SpellName, TerrestrialSpell>,
    pub(in crate::sorcery::circles) celestial_control_spell_name: SpellName,
    pub(in crate::sorcery::circles) celestial_control_spell: CelestialSpell,
    pub(in crate::sorcery::circles) celestial_spells: HashMap<SpellName, CelestialSpell>,
}

impl<'source> CelestialCircleSorcererMemo {
    pub fn as_ref(&'source self) -> CelestialCircleSorcerer<'source> {
        CelestialCircleSorcerer {
            archetypes: self
                .archetypes
                .iter()
                .map(|(k, (archetype, merits))| {
                    (
                        k.as_str(),
                        (archetype, merits.iter().map(|(k, v)| (k.as_str(), v)).collect()),
                    )
                })
                .collect(),
            circle_archetypes: [
                self.circle_archetypes[0].as_str(),
                self.circle_archetypes[1].as_str(),
            ],
            shaping_ritual_names: [
                self.shaping_ritual_names[0].as_str(),
                self.shaping_ritual_names[1].as_str(),
            ],
            shaping_rituals: [&self.shaping_rituals[0], &self.shaping_rituals[1]],

            terrestrial_control_spell_name: self.terrestrial_control_spell_name.as_str(),
            terrestrial_control_spell: &self.terrestrial_control_spell,
            terrestrial_spells: self
                .terrestrial_spells
                .iter()
                .map(|(k, v)| (k.as_str(), v))
                .collect(),
            celestial_control_spell_name: self.celestial_control_spell_name.as_str(),
            celestial_control_spell: &self.celestial_control_spell,
            celestial_spells: self
                .celestial_spells
                .iter()
                .map(|(k, v)| (k.as_str(), v))
                .collect(),
        }
    }
}
