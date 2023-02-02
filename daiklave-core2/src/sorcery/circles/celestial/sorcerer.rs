use std::collections::{hash_map::Entry, HashMap};

use crate::{
    charms::CharmError,
    merits::merit::SorceryArchetypeMeritDetails,
    sorcery::{
        circles::{
            solar::{sorcerer::SolarCircleSorcerer, AddSolarSorcery},
            sorcery_circle::SorceryCircle,
            terrestrial::TerrestrialSpell,
        },
        spell::Spell,
        ShapingRitualDetails, SorceryArchetypeDetails, SorceryArchetype, SorceryError, ShapingRitual,
    },
    CharacterMutationError,
};

use super::{sorcerer_memo::CelestialCircleSorcererMemo, spell::CelestialSpell};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CelestialCircleSorcerer<'source> {
    pub(crate) archetypes: HashMap<
        &'source str,
        (
            &'source SorceryArchetypeDetails,
            HashMap<&'source str, &'source SorceryArchetypeMeritDetails>,
        ),
    >,
    pub(crate) circle_archetypes: [&'source str; 2],
    pub(crate) shaping_ritual_names: [&'source str; 2],
    pub(crate) shaping_rituals: [&'source ShapingRitualDetails; 2],
    pub(crate) terrestrial_control_spell_name: &'source str,
    pub(crate) terrestrial_control_spell: &'source TerrestrialSpell,
    pub(crate) terrestrial_spells: HashMap<&'source str, &'source TerrestrialSpell>,
    pub(crate) celestial_control_spell_name: &'source str,
    pub(crate) celestial_control_spell: &'source CelestialSpell,
    pub(crate) celestial_spells: HashMap<&'source str, &'source CelestialSpell>,
}

impl<'source> From<&'source CelestialCircleSorcererMemo> for CelestialCircleSorcerer<'source> {
    fn from(memo: &'source CelestialCircleSorcererMemo) -> Self {
        CelestialCircleSorcerer {
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
            ],
            shaping_ritual_names: [
                memo.shaping_ritual_names[0].as_str(),
                memo.shaping_ritual_names[1].as_str(),
            ],
            shaping_rituals: [&memo.shaping_rituals[0], &memo.shaping_rituals[1]],
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
        }
    }
}

impl<'view, 'source> CelestialCircleSorcerer<'source> {
    pub fn archetype(
        &'view self,
        name: &str,
    ) -> Option<SorceryArchetype<'view, 'source>> {
        if self.circle_archetypes.contains(&name) {
            self.archetypes
                .get_key_value(name)
                .map(
                    |(archetype_name, (archetype, merits))| SorceryArchetype {
                        archetype_name: *archetype_name,
                        archetype: *archetype,
                        merits,
                    },
                )
        } else {
            None
        }
    }

    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> Option<ShapingRitual<'source>> {
        match circle {
            SorceryCircle::Terrestrial => Some(ShapingRitual {
                archetype_name: self.circle_archetypes[0],
                summary: self.shaping_ritual_names[0],
                details: self.shaping_rituals[0],
            }),
            SorceryCircle::Celestial => Some(ShapingRitual {
                archetype_name: self.circle_archetypes[1],
                summary: self.shaping_ritual_names[1],
                details: self.shaping_rituals[1],
            }),
            SorceryCircle::Solar => None,
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> Option<Spell<'source>> {
        match circle {
            SorceryCircle::Terrestrial => Some(Spell::Terrestrial(
                self.terrestrial_control_spell_name,
                self.terrestrial_control_spell,
            )),
            SorceryCircle::Celestial => Some(Spell::Celestial(
                self.celestial_control_spell_name,
                self.celestial_control_spell,
            )),
            SorceryCircle::Solar => None,
        }
    }

    pub fn upgrade(
        &self,
        add_solar: &'source AddSolarSorcery,
    ) -> Result<SolarCircleSorcerer<'source>, CharacterMutationError> {
        if add_solar.shaping_ritual_summary == self.shaping_ritual_names[0]
            || add_solar.shaping_ritual_summary == self.shaping_ritual_names[1]
        {
            return Err(CharacterMutationError::SorceryError(
                SorceryError::DuplicateShapingRitual,
            ));
        }

        let mut archetypes = self.archetypes.clone();

        if let Entry::Vacant(e) = archetypes.entry(add_solar.archetype_name.as_str()) {
            if let Some(new_archetype) = &add_solar.archetype {
                e.insert((new_archetype, HashMap::new()));
            } else {
                return Err(CharacterMutationError::SorceryError(
                    SorceryError::MissingArchetype,
                ));
            }
        }

        Ok(SolarCircleSorcerer {
            archetypes,
            circle_archetypes: [
                self.circle_archetypes[0],
                self.circle_archetypes[1],
                add_solar.archetype_name.as_str(),
            ],
            shaping_ritual_names: [
                self.shaping_ritual_names[0],
                self.shaping_ritual_names[1],
                add_solar.shaping_ritual_summary.as_str(),
            ],
            shaping_rituals: [
                self.shaping_rituals[0],
                self.shaping_rituals[1],
                &add_solar.shaping_ritual,
            ],
            terrestrial_control_spell_name: self.terrestrial_control_spell_name,
            terrestrial_control_spell: self.terrestrial_control_spell,
            terrestrial_spells: self.terrestrial_spells.clone(),
            celestial_control_spell_name: self.celestial_control_spell_name,
            celestial_control_spell: self.celestial_control_spell,
            celestial_spells: self.celestial_spells.clone(),
            solar_control_spell_name: add_solar.control_spell_name.as_str(),
            solar_control_spell: &add_solar.control_spell,
            solar_spells: HashMap::new(),
        })
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
        }
    }

    pub fn spells_iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        std::iter::once(self.terrestrial_control_spell_name)
            .chain(self.terrestrial_spells.keys().copied())
            .chain(std::iter::once(self.celestial_control_spell_name))
            .chain(self.celestial_spells.keys().copied())
    }

    pub fn add_terrestrial_spell(
        &mut self,
        name: &'source str,
        spell: &'source TerrestrialSpell,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.terrestrial_control_spell_name == name
            || self.celestial_control_spell_name == name
            || self.terrestrial_spells.contains_key(name)
            || self.celestial_spells.contains_key(name)
        {
            Err(CharacterMutationError::CharmError(
                CharmError::DuplicateCharm,
            ))
        } else {
            self.terrestrial_spells.insert(name, spell);
            Ok(self)
        }
    }

    pub fn add_celestial_spell(
        &mut self,
        name: &'source str,
        spell: &'source CelestialSpell,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.terrestrial_control_spell_name == name
            || self.celestial_control_spell_name == name
            || self.terrestrial_spells.contains_key(name)
            || self.celestial_spells.contains_key(name)
        {
            Err(CharacterMutationError::CharmError(
                CharmError::DuplicateCharm,
            ))
        } else {
            self.celestial_spells.insert(name, spell);
            Ok(self)
        }
    }

    pub fn remove_spell(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        if self.terrestrial_spells.remove(name).is_none()
            && self.celestial_spells.remove(name).is_none()
        {
            if name == self.celestial_control_spell_name
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

impl<'view, 'source> From<&'view SolarCircleSorcerer<'source>>
    for CelestialCircleSorcerer<'source>
{
    fn from(solar: &'view SolarCircleSorcerer<'source>) -> Self {
        let mut archetypes = solar.archetypes.clone();
        if solar.circle_archetypes[2] != solar.circle_archetypes[0]
            && solar.circle_archetypes[2] != solar.circle_archetypes[1]
        {
            archetypes.remove(&solar.circle_archetypes[2]);
        }

        Self {
            archetypes,
            circle_archetypes: [solar.circle_archetypes[0], solar.circle_archetypes[1]],
            shaping_ritual_names: [solar.shaping_ritual_names[0], solar.shaping_ritual_names[1]],
            shaping_rituals: [solar.shaping_rituals[0], solar.shaping_rituals[1]],
            terrestrial_control_spell_name: solar.terrestrial_control_spell_name,
            terrestrial_control_spell: solar.terrestrial_control_spell,
            terrestrial_spells: solar.terrestrial_spells.clone(),
            celestial_control_spell_name: solar.celestial_control_spell_name,
            celestial_control_spell: solar.celestial_control_spell,
            celestial_spells: solar.celestial_spells.clone(),
        }
    }
}
