use std::collections::HashMap;

use crate::{
    charms::CharmError,
    merits::merit::SorceryArchetypeMeritDetails,
    sorcery::{
        circles::{
            celestial::CelestialSpell, sorcery_circle::SorceryCircle, terrestrial::TerrestrialSpell,
        },
        spell::{Spell, SpellMutation},
        ShapingRitualDetails, SorceryArchetypeDetails, SorceryArchetypeWithMerits, SorceryError, ShapingRitual,
    },
    CharacterMutationError,
};

use super::{sorcerer_memo::SolarCircleSorcererMemo, SolarSpell};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SolarCircleSorcerer<'source> {
    pub(crate) archetypes: HashMap<
        &'source str,
        (
            &'source SorceryArchetypeDetails,
            HashMap<&'source str, &'source SorceryArchetypeMeritDetails>,
        ),
    >,
    pub(crate) circle_archetypes: [&'source str; 3],
    pub(crate) shaping_ritual_names: [&'source str; 3],
    pub(crate) shaping_rituals: [&'source ShapingRitualDetails; 3],
    pub(crate) terrestrial_control_spell_name: &'source str,
    pub(crate) terrestrial_control_spell: &'source TerrestrialSpell,
    pub(crate) terrestrial_spells: HashMap<&'source str, &'source TerrestrialSpell>,
    pub(crate) celestial_control_spell_name: &'source str,
    pub(crate) celestial_control_spell: &'source CelestialSpell,
    pub(crate) celestial_spells: HashMap<&'source str, &'source CelestialSpell>,
    pub(crate) solar_control_spell_name: &'source str,
    pub(crate) solar_control_spell: &'source SolarSpell,
    pub(crate) solar_spells: HashMap<&'source str, &'source SolarSpell>,
}

impl<'source> From<&'source SolarCircleSorcererMemo> for SolarCircleSorcerer<'source> {
    fn from(memo: &'source SolarCircleSorcererMemo) -> Self {
        SolarCircleSorcerer {
            archetypes: memo
                .archetypes
                .iter()
                .map(|(archetype_name, (archetype_details, merits_map))| {
                    (
                        archetype_name.as_str(),
                        (
                            archetype_details,
                            merits_map
                                .iter()
                                .map(|(merit_name, merit_details)| {
                                    (merit_name.as_str(), merit_details)
                                })
                                .collect(),
                        ),
                    )
                })
                .collect(),
            circle_archetypes: [
                memo.circle_archetypes[0].as_str(),
                memo.circle_archetypes[1].as_str(),
                memo.circle_archetypes[2].as_str(),
            ],
            shaping_ritual_names: [
                memo.shaping_ritual_names[0].as_str(),
                memo.shaping_ritual_names[1].as_str(),
                memo.shaping_ritual_names[2].as_str(),
            ],
            shaping_rituals: [
                &memo.shaping_rituals[0],
                &memo.shaping_rituals[1],
                &memo.shaping_rituals[2],
            ],
            terrestrial_control_spell_name: &memo.terrestrial_control_spell_name,
            terrestrial_control_spell: &memo.terrestrial_control_spell,
            terrestrial_spells: memo
                .terrestrial_spells
                .iter()
                .map(|(spell_name, spell)| (spell_name.as_str(), spell))
                .collect(),
            celestial_control_spell_name: &memo.celestial_control_spell_name,
            celestial_control_spell: &memo.celestial_control_spell,
            celestial_spells: memo
                .celestial_spells
                .iter()
                .map(|(spell_name, spell)| (spell_name.as_str(), spell))
                .collect(),
            solar_control_spell_name: &memo.solar_control_spell_name,
            solar_control_spell: &memo.solar_control_spell,
            solar_spells: memo
                .solar_spells
                .iter()
                .map(|(spell_name, spell)| (spell_name.as_str(), spell))
                .collect(),
        }
    }
}

impl<'view, 'source> SolarCircleSorcerer<'source> {
    pub fn archetype(
        &'view self,
        name: &str,
    ) -> Option<SorceryArchetypeWithMerits<'view, 'source>> {
        if self.circle_archetypes.contains(&name) {
            self.archetypes
                .get_key_value(name)
                .map(|(name, (archetype, merits))| SorceryArchetypeWithMerits {
                    archetype_name: *name,
                    archetype: *archetype,
                    merits,
                })
        } else {
            None
        }
    }

    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> ShapingRitual<'source> {
        match circle {
            SorceryCircle::Terrestrial => ShapingRitual {
                archetype_name: self.circle_archetypes[0],
                summary: self.shaping_ritual_names[0],
                details: self.shaping_rituals[0],
            },
            SorceryCircle::Celestial => ShapingRitual {
                archetype_name: self.circle_archetypes[1],
                summary: self.shaping_ritual_names[1],
                details: self.shaping_rituals[1],
            },
            SorceryCircle::Solar => ShapingRitual {
                archetype_name: self.circle_archetypes[2],
                summary: self.shaping_ritual_names[2],
                details: self.shaping_rituals[2],
            },
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> Spell<'source> {
        match circle {
            SorceryCircle::Terrestrial => Spell::Terrestrial(
                self.terrestrial_control_spell_name,
                self.terrestrial_control_spell,
            ),
            SorceryCircle::Celestial => Spell::Celestial(
                self.celestial_control_spell_name,
                self.celestial_control_spell,
            ),
            SorceryCircle::Solar => {
                Spell::Solar(self.solar_control_spell_name, self.solar_control_spell)
            }
        }
    }

    pub fn get_spell(&self, name: &str) -> Option<(Spell<'source>, bool)> {
        if name == self.terrestrial_control_spell_name {
            Some((
                Spell::Terrestrial(
                    self.terrestrial_control_spell_name,
                    self.terrestrial_control_spell,
                ),
                true,
            ))
        } else if name == self.celestial_control_spell_name {
            Some((
                Spell::Celestial(
                    self.celestial_control_spell_name,
                    self.celestial_control_spell,
                ),
                true,
            ))
        } else if name == self.solar_control_spell_name {
            Some((
                Spell::Solar(self.solar_control_spell_name, self.solar_control_spell),
                true,
            ))
        } else {
            self.terrestrial_spells
                .get_key_value(name)
                .map(|(terrestrial_spell_name, terrestrial_spell)| {
                    (
                        Spell::Terrestrial(terrestrial_spell_name, terrestrial_spell),
                        false,
                    )
                })
                .or_else(|| {
                    self.celestial_spells.get_key_value(name).map(
                        |(celestial_spell_name, celestial_spell)| {
                            (
                                Spell::Celestial(celestial_spell_name, celestial_spell),
                                false,
                            )
                        },
                    )
                })
                .or_else(|| {
                    self.solar_spells
                        .get_key_value(name)
                        .map(|(solar_spell_name, solar_spell)| {
                            (Spell::Solar(solar_spell_name, solar_spell), false)
                        })
                })
        }
    }

    pub fn spells_iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        std::iter::once(self.terrestrial_control_spell_name)
            .chain(self.terrestrial_spells.keys().copied())
            .chain(std::iter::once(self.celestial_control_spell_name))
            .chain(self.celestial_spells.keys().copied())
    }

    pub fn add_spell(
        &mut self,
        name: &'source str,
        spell: &'source SpellMutation,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.terrestrial_control_spell_name == name
            || self.celestial_control_spell_name == name
            || self.solar_control_spell_name == name
            || self.terrestrial_spells.contains_key(name)
            || self.celestial_spells.contains_key(name)
            || self.solar_spells.contains_key(name)
        {
            Err(CharacterMutationError::CharmError(
                CharmError::DuplicateCharm,
            ))
        } else {
            match spell {
                SpellMutation::Terrestrial(terrestrial_spell) => {
                    self.terrestrial_spells.insert(name, terrestrial_spell);
                }
                SpellMutation::Celestial(celestial_spell) => {
                    self.celestial_spells.insert(name, celestial_spell);
                }
                SpellMutation::Solar(solar_spell) => {
                    self.solar_spells.insert(name, solar_spell);
                }
            }
            Ok(self)
        }
    }

    pub fn remove_spell(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        if self.terrestrial_spells.remove(name).is_none()
            && self.celestial_spells.remove(name).is_none()
            && self.solar_spells.remove(name).is_none()
        {
            if name == self.solar_control_spell_name
                || name == self.celestial_control_spell_name
                || name == self.terrestrial_control_spell_name
            {
                Err(CharacterMutationError::SorceryError(
                    SorceryError::RemoveControlSpell,
                ))
            } else {
                Err(CharacterMutationError::CharmError(CharmError::NotFound))
            }
        } else {
            Ok(self)
        }
    }
}
