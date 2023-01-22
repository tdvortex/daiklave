use std::collections::HashMap;

use crate::sorcery::{
    circles::{
        celestial::CelestialSpell, sorcery_circle::SorceryCircle, terrestrial::TerrestrialSpell,
    },
    spell::{Spell, SpellId},
    ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId, SorceryArchetypeMerit,
    SorceryArchetypeMeritId, SorceryArchetypeWithMerits,
};

use super::{sorcerer_memo::SolarCircleSorcererMemo, SolarSpell};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SolarCircleSorcerer<'source> {
    pub(crate) archetypes: HashMap<
        SorceryArchetypeId,
        (
            &'source SorceryArchetype,
            HashMap<SorceryArchetypeMeritId, &'source SorceryArchetypeMerit>,
        ),
    >,
    pub(crate) circle_archetypes: [SorceryArchetypeId; 3],
    pub(crate) shaping_ritual_ids: [ShapingRitualId; 3],
    pub(crate) shaping_rituals: [&'source ShapingRitual; 3],
    pub(crate) terrestrial_control_spell_id: SpellId,
    pub(crate) terrestrial_control_spell: &'source TerrestrialSpell,
    pub(crate) terrestrial_spells: HashMap<SpellId, &'source TerrestrialSpell>,
    pub(crate) celestial_control_spell_id: SpellId,
    pub(crate) celestial_control_spell: &'source CelestialSpell,
    pub(crate) celestial_spells: HashMap<SpellId, &'source CelestialSpell>,
    pub(crate) solar_control_spell_id: SpellId,
    pub(crate) solar_control_spell: &'source SolarSpell,
    pub(crate) solar_spells: HashMap<SpellId, &'source SolarSpell>,
}

impl<'view, 'source> SolarCircleSorcerer<'source> {
    pub fn as_memo(&self) -> SolarCircleSorcererMemo {
        SolarCircleSorcererMemo {
            archetypes: self
                .archetypes
                .iter()
                .map(|(k, (archetype, merits))| {
                    (
                        *k,
                        (
                            (*archetype).to_owned(),
                            merits.iter().map(|(k, v)| (*k, (*v).to_owned())).collect(),
                        ),
                    )
                })
                .collect(),
            circle_archetypes: self.circle_archetypes,
            shaping_ritual_ids: self.shaping_ritual_ids,
            shaping_rituals: { self.shaping_rituals.map(|ptr| ptr.to_owned()) },
            terrestrial_control_spell_id: self.terrestrial_control_spell_id,
            terrestrial_control_spell: self.terrestrial_control_spell.to_owned(),
            terrestrial_spells: self
                .terrestrial_spells
                .iter()
                .map(|(k, v)| (*k, (*v).to_owned()))
                .collect(),
            celestial_control_spell_id: self.celestial_control_spell_id,
            celestial_control_spell: self.celestial_control_spell.to_owned(),
            celestial_spells: self
                .celestial_spells
                .iter()
                .map(|(k, v)| (*k, (*v).to_owned()))
                .collect(),
            solar_control_spell_id: self.solar_control_spell_id,
            solar_control_spell: self.solar_control_spell.to_owned(),
            solar_spells: self
                .solar_spells
                .iter()
                .map(|(k, v)| (*k, (*v).to_owned()))
                .collect(),
        }
    }

    pub fn archetype(
        &'view self,
        id: SorceryArchetypeId,
    ) -> Option<SorceryArchetypeWithMerits<'view, 'source>> {
        if self.circle_archetypes.contains(&id) {
            self.archetypes
                .get(&id)
                .map(|(archetype, merits)| (*archetype, merits))
        } else {
            None
        }
    }

    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> (ShapingRitualId, &'source ShapingRitual) {
        match circle {
            SorceryCircle::Terrestrial => (self.shaping_ritual_ids[0], self.shaping_rituals[0]),
            SorceryCircle::Celestial => (self.shaping_ritual_ids[1], self.shaping_rituals[1]),
            SorceryCircle::Solar => (self.shaping_ritual_ids[2], self.shaping_rituals[2]),
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> (SpellId, Spell<'source>) {
        match circle {
            SorceryCircle::Terrestrial => (
                self.terrestrial_control_spell_id,
                Spell::Terrestrial(self.terrestrial_control_spell),
            ),
            SorceryCircle::Celestial => (
                self.celestial_control_spell_id,
                Spell::Celestial(self.celestial_control_spell),
            ),
            SorceryCircle::Solar => (
                self.solar_control_spell_id,
                Spell::Solar(self.solar_control_spell),
            ),
        }
    }

    pub fn get_spell(&self, spell_id: SpellId) -> Option<(Spell<'source>, bool)> {
        if spell_id == self.terrestrial_control_spell_id {
            Some((Spell::Terrestrial(self.terrestrial_control_spell), true))
        } else if spell_id == self.celestial_control_spell_id {
            Some((Spell::Celestial(self.celestial_control_spell), true))
        } else if spell_id == self.solar_control_spell_id {
            Some((Spell::Solar(self.solar_control_spell), true))
        } else {
            self
            .terrestrial_spells
            .get(&spell_id)
            .map(|terrestrial_spell| (Spell::Terrestrial(*terrestrial_spell), false))
            .or_else(|| self.celestial_spells.get(&spell_id).map(|celestial_spell| (Spell::Celestial(*celestial_spell), false)))
            .or_else(|| self.solar_spells.get(&spell_id).map(|solar_spell| (Spell::Solar(*solar_spell), false)))
            
        }
    }

    pub fn spells_iter(&self) -> impl Iterator<Item = SpellId> + '_ {
        std::iter::once(self.terrestrial_control_spell_id)
        .chain(self.terrestrial_spells.keys().copied())
        .chain(std::iter::once(self.celestial_control_spell_id))
        .chain(self.celestial_spells.keys().copied())
    }
}
