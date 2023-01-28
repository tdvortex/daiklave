use std::collections::HashMap;

use crate::{
    charms::CharmError,
    sorcery::{
        circles::{
            celestial::CelestialSpell, sorcery_circle::SorceryCircle, terrestrial::TerrestrialSpell,
        },
        spell::{Spell, SpellMutation},
        ShapingRitual, SorceryArchetype, SorceryArchetypeMerit,
        SorceryArchetypeWithMerits, SorceryError,
    },
    CharacterMutationError,
};

use super::{sorcerer_memo::SolarCircleSorcererMemo, SolarSpell};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SolarCircleSorcerer<'source> {
    pub(crate) archetypes: HashMap<
        &'source str,
        (
            &'source SorceryArchetype,
            HashMap<&'source str, &'source SorceryArchetypeMerit>,
        ),
    >,
    pub(crate) circle_archetypes: [&'source str; 3],
    pub(crate) shaping_ritual_names: [&'source str; 3],
    pub(crate) shaping_rituals: [&'source ShapingRitual; 3],
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

impl<'view, 'source> SolarCircleSorcerer<'source> {
    pub fn as_memo(&self) -> SolarCircleSorcererMemo {
        SolarCircleSorcererMemo {
            archetypes: self
                .archetypes
                .iter()
                .map(|(k, (archetype, merits))| {
                    (
                        (*k).to_owned(),
                        (
                            (*archetype).to_owned(),
                            merits.iter().map(|(k, v)| ((*k).to_owned(), (*v).to_owned())).collect(),
                        ),
                    )
                })
                .collect(),
            circle_archetypes: [
                (*self.circle_archetypes[0]).to_owned(),
                (*self.circle_archetypes[1]).to_owned(),
                (*self.circle_archetypes[2]).to_owned(),
            ],
            shaping_ritual_names: [
                (*self.shaping_ritual_names[0]).to_owned(),
                (*self.shaping_ritual_names[1]).to_owned(),
                (*self.shaping_ritual_names[2]).to_owned(),
            ],
            shaping_rituals: { self.shaping_rituals.map(|ptr| ptr.to_owned()) },
            terrestrial_control_spell_name: (*self.terrestrial_control_spell_name).to_owned(),
            terrestrial_control_spell: self.terrestrial_control_spell.to_owned(),
            terrestrial_spells: self
                .terrestrial_spells
                .iter()
                .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
                .collect(),
            celestial_control_spell_name: (*self.celestial_control_spell_name).to_owned(),
            celestial_control_spell: self.celestial_control_spell.to_owned(),
            celestial_spells: self
                .celestial_spells
                .iter()
                .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
                .collect(),
            solar_control_spell_name: (*self.solar_control_spell_name).to_owned(),
            solar_control_spell: self.solar_control_spell.to_owned(),
            solar_spells: self
                .solar_spells
                .iter()
                .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
                .collect(),
        }
    }

    pub fn archetype(
        &'view self,
        name: &str,
    ) -> Option<SorceryArchetypeWithMerits<'view, 'source>> {
        if self.circle_archetypes.contains(&name) {
            self.archetypes
                .get_key_value(name)
                .map(|(name, (archetype, merits))| (*name, *archetype, merits))
        } else {
            None
        }
    }

    pub fn shaping_ritual(&self, circle: SorceryCircle) -> (&'source str, &'source ShapingRitual) {
        match circle {
            SorceryCircle::Terrestrial => (self.shaping_ritual_names[0], self.shaping_rituals[0]),
            SorceryCircle::Celestial => (self.shaping_ritual_names[1], self.shaping_rituals[1]),
            SorceryCircle::Solar => (self.shaping_ritual_names[2], self.shaping_rituals[2]),
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
