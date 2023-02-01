use std::collections::{hash_map::Entry, HashMap};

use crate::{
    charms::CharmError,
    merits::merit_new::SorceryArchetypeMeritDetails,
    sorcery::{
        circles::celestial::{sorcerer::CelestialCircleSorcerer, AddCelestialSorcery},
        spell::Spell,
        ShapingRitualDetails, SorceryArchetypeDetails, SorceryArchetypeWithMerits, SorceryError,
    },
    CharacterMutationError,
};

use super::{sorcerer_memo::TerrestrialCircleSorcererMemo, TerrestrialSpell};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerrestrialCircleSorcerer<'source> {
    pub archetype_name: &'source str,
    pub archetype: &'source SorceryArchetypeDetails,
    pub archetype_merits: HashMap<&'source str, &'source SorceryArchetypeMeritDetails>,
    pub shaping_ritual_name: &'source str,
    pub shaping_ritual: &'source ShapingRitualDetails,
    pub control_spell_name: &'source str,
    pub control_spell: &'source TerrestrialSpell,
    pub other_spells: HashMap<&'source str, &'source TerrestrialSpell>,
}

impl<'source> From<&'source TerrestrialCircleSorcererMemo> for TerrestrialCircleSorcerer<'source> {
    fn from(memo: &'source TerrestrialCircleSorcererMemo) -> Self {
        Self {
            archetype_name: &memo.archetype_name,
            archetype: &memo.archetype,
            archetype_merits: memo
                .archetype_merits
                .iter()
                .map(|(merit_name, merit_details)| (merit_name.as_str(), merit_details))
                .collect(),
            shaping_ritual_name: &memo.shaping_ritual_name,
            shaping_ritual: &memo.shaping_ritual,
            control_spell_name: &memo.control_spell_name,
            control_spell: &memo.control_spell,
            other_spells: memo
                .other_spells
                .iter()
                .map(|(spell_name, spell)| (spell_name.as_str(), spell))
                .collect(),
        }
    }
}

impl<'view, 'source> TerrestrialCircleSorcerer<'source> {
    pub fn archetype(
        &'view self,
        name: &str,
    ) -> Option<SorceryArchetypeWithMerits<'view, 'source>> {
        if name == self.archetype_name {
            Some(SorceryArchetypeWithMerits {
                archetype_name: self.archetype_name,
                archetype: self.archetype,
                merits: &self.archetype_merits,
            })
        } else {
            None
        }
    }

    pub fn shaping_ritual(&self) -> (&'source str, &'source ShapingRitualDetails) {
        (self.shaping_ritual_name, self.shaping_ritual)
    }

    pub fn control_spell(&self) -> Spell<'source> {
        Spell::Terrestrial(self.control_spell_name, self.control_spell)
    }

    pub fn upgrade(
        &self,
        add_sorcery: &'source AddCelestialSorcery,
    ) -> Result<CelestialCircleSorcerer<'source>, CharacterMutationError> {
        if add_sorcery.shaping_ritual_summary == self.shaping_ritual_name {
            return Err(CharacterMutationError::SorceryError(
                SorceryError::DuplicateShapingRitual,
            ));
        }

        let mut archetypes = HashMap::new();

        archetypes.insert(
            self.archetype_name,
            (self.archetype, self.archetype_merits.clone()),
        );

        if let Entry::Vacant(e) = archetypes.entry(add_sorcery.archetype_name.as_str()) {
            if let Some(new_archetype) = &add_sorcery.archetype {
                e.insert((new_archetype, HashMap::new()));
            } else {
                return Err(CharacterMutationError::SorceryError(
                    SorceryError::MissingArchetype,
                ));
            }
        }

        Ok(CelestialCircleSorcerer {
            archetypes,
            circle_archetypes: [self.archetype_name, add_sorcery.archetype_name.as_str()],
            shaping_ritual_names: [
                self.shaping_ritual_name,
                add_sorcery.shaping_ritual_summary.as_str(),
            ],
            shaping_rituals: [self.shaping_ritual, &add_sorcery.shaping_ritual],
            terrestrial_control_spell_name: self.control_spell_name,
            terrestrial_control_spell: self.control_spell,
            terrestrial_spells: self.other_spells.clone(),
            celestial_control_spell_name: add_sorcery.control_spell_name.as_str(),
            celestial_control_spell: &add_sorcery.control_spell,
            celestial_spells: HashMap::new(),
        })
    }

    pub fn get_spell(&self, name: &str) -> Option<(Spell<'source>, bool)> {
        if name == self.control_spell_name {
            Some((
                Spell::Terrestrial(self.control_spell_name, self.control_spell),
                true,
            ))
        } else {
            self.other_spells
                .get_key_value(name)
                .map(|(spell_name, terrestrial_spell)| {
                    (Spell::Terrestrial(*spell_name, terrestrial_spell), false)
                })
        }
    }

    pub fn spells_iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        std::iter::once(self.control_spell_name).chain(self.other_spells.keys().copied())
    }

    pub fn add_terrestrial_spell(
        &mut self,
        name: &'source str,
        spell: &'source TerrestrialSpell,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.control_spell_name == name || self.other_spells.contains_key(name) {
            Err(CharacterMutationError::CharmError(
                CharmError::DuplicateCharm,
            ))
        } else {
            self.other_spells.insert(name, spell);
            Ok(self)
        }
    }

    pub fn remove_spell(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        if self.other_spells.remove(name).is_none() {
            if name == self.control_spell_name {
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

impl<'view, 'source> TryFrom<&'view CelestialCircleSorcerer<'source>>
    for TerrestrialCircleSorcerer<'source>
{
    type Error = SorceryError;

    fn try_from(celestial: &'view CelestialCircleSorcerer<'source>) -> Result<Self, Self::Error> {
        Ok(Self {
            archetype_name: celestial.circle_archetypes[0],
            archetype: celestial
                .archetypes
                .get(&celestial.circle_archetypes[0])
                .map(|(archetype, _merits)| *archetype)
                .ok_or(SorceryError::MissingArchetype)?,
            archetype_merits: celestial
                .archetypes
                .get(&celestial.circle_archetypes[0])
                .map(|(_archetype, merits)| merits.to_owned())
                .ok_or(SorceryError::MissingArchetype)?,
            shaping_ritual_name: celestial.shaping_ritual_names[0],
            shaping_ritual: celestial.shaping_rituals[0],
            control_spell_name: celestial.terrestrial_control_spell_name,
            control_spell: celestial.terrestrial_control_spell,
            other_spells: celestial.terrestrial_spells.clone(),
        })
    }
}
