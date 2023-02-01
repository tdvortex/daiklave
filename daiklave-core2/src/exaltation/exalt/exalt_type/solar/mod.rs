/// Traits relating to specific Solar castes.
pub mod caste;

/// A builder path for constructing a new Solar.
pub mod builder;

/// Details of a Solar Charm.
pub mod charm;

mod anima_effect;
mod error;
mod memo;
mod set;
mod sorcery;

use std::collections::{hash_map::Entry, HashMap, HashSet};

pub use error::SolarError;
pub(crate) use memo::SolarMemo;
pub use set::SetSolar;
pub(crate) use sorcery::{SolarSorcererMemo, SolarSorcererView};

use crate::{
    abilities::AbilityName,
    charms::{charm::Charm, CharmError},
    exaltation::exalt::{AnimaEffect, Limit},
    experience::ExperiencePool,
    merits::merit_new::{MeritError, SorceryArchetypeMeritDetails},
    sorcery::{
        circles::{
            celestial::AddCelestialSorcery,
            solar::AddSolarSorcery,
            terrestrial::{sorcerer::TerrestrialCircleSorcerer},
        },
        spell::SpellMutation,
        SorceryError, AddTerrestrialSorcery,
    },
    CharacterMutationError,
};

use self::{
    anima_effect::{SOLAR_ONE, SOLAR_TWO},
    builder::SolarBuilder,
    caste::SolarCaste,
    charm::SolarCharm,
};

/// Traits which are unique to being a Solar Exalted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Solar<'source> {
    pub(crate) caste: SolarCaste<'source>,
    pub(crate) favored_abilities: [AbilityName; 5],
    pub(crate) experience: ExperiencePool,
    pub(crate) sorcery: Option<SolarSorcererView<'source>>,
    pub(crate) limit: Limit<'source>,
    pub(crate) solar_charms: Vec<(&'source str, &'source SolarCharm)>,
}

impl<'source> Solar<'source> {
    /// Starts building a set of Solar traits
    pub fn builder() -> SolarBuilder {
        SolarBuilder {
            limit_trigger: None,
        }
    }
    
    /// Returns True if the ability is a caste ability for the charcter. Note
    /// that MartialArts is a caste ability if and only if Brawl is a caste
    /// ability.
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        self.caste.has_caste_ability(ability)
    }

    /// Returns the Solar's supernal ability.
    pub fn supernal_ability(&self) -> AbilityName {
        self.caste.supernal_ability()
    }

    /// Returns True if the ability is a favored ability for the charcter. Note
    /// that MartialArts is a favored ability if and only if Brawl is a favored
    /// ability.
    pub fn has_favored_ability(&self, ability: AbilityName) -> bool {
        let search_ability = if ability == AbilityName::MartialArts {
            AbilityName::Brawl
        } else {
            ability
        };

        self.favored_abilities.iter().any(|&a| a == search_ability)
    }

    /// The anima effects which the Solar possesses.
    pub fn anima_effects(&self) -> impl Iterator<Item = AnimaEffect> {
        [SOLAR_ONE, SOLAR_TWO]
            .into_iter()
            .chain(self.caste.anima_effects().into_iter())
    }

    /// The current state of the Solar's Great Curse.
    pub fn limit(&self) -> Limit<'source> {
        self.limit
    }

    /// The Solar's pool of Solar Experience
    pub fn experience(&self) -> ExperiencePool {
        self.experience
    }

    pub(crate) fn add_terrestrial_sorcery(
        &mut self,
        add_terrestrial: &'source AddTerrestrialSorcery,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.sorcery.is_none() {
            self.sorcery = Some(SolarSorcererView::Terrestrial(TerrestrialCircleSorcerer {
                archetype_name: &add_terrestrial.archetype_name,
                archetype: &add_terrestrial.archetype,
                archetype_merits: HashMap::new(),
                shaping_ritual_name: &add_terrestrial.shaping_ritual_summary,
                shaping_ritual: &add_terrestrial.shaping_ritual,
                control_spell_name: &add_terrestrial.control_spell_name,
                control_spell: &add_terrestrial.control_spell,
                other_spells: HashMap::new(),
            }));
            Ok(self)
        } else {
            Err(CharacterMutationError::SorceryError(
                SorceryError::CircleSequence,
            ))
        }
    }

    pub(crate) fn remove_terrestrial_sorcery(
        &mut self,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self.sorcery {
            Some(SolarSorcererView::Terrestrial(_)) => {
                self.sorcery = None;
                Ok(self)
            }
            _ => Err(CharacterMutationError::SorceryError(
                SorceryError::CircleSequence,
            )),
        }
    }

    pub(crate) fn add_celestial_sorcery(
        &mut self,
        add_celestial: &'source AddCelestialSorcery,
    ) -> Result<&mut Self, CharacterMutationError> {
        let celestial = match &self.sorcery {
            Some(SolarSorcererView::Terrestrial(terrestrial)) => terrestrial.upgrade(add_celestial),
            _ => Err(CharacterMutationError::SorceryError(
                SorceryError::CircleSequence,
            )),
        }?;
        self.sorcery = Some(SolarSorcererView::Celestial(celestial));
        Ok(self)
    }

    pub(crate) fn remove_celestial_sorcery(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if let Some(SolarSorcererView::Celestial(celestial)) = &mut self.sorcery {
            self.sorcery = Some(SolarSorcererView::Terrestrial(
                (&*celestial)
                    .try_into()
                    .map_err(CharacterMutationError::SorceryError)?,
            ));
            Ok(self)
        } else {
            Err(CharacterMutationError::SorceryError(
                SorceryError::CircleSequence,
            ))
        }
    }

    pub(crate) fn add_solar_sorcery(
        &mut self,
        add_solar: &'source AddSolarSorcery,
    ) -> Result<&mut Self, CharacterMutationError> {
        let solar = match &self.sorcery {
            Some(SolarSorcererView::Celestial(celestial)) => celestial.upgrade(add_solar),
            _ => Err(CharacterMutationError::SorceryError(
                SorceryError::CircleSequence,
            )),
        }?;
        self.sorcery = Some(SolarSorcererView::Solar(solar));
        Ok(self)
    }

    pub(crate) fn remove_solar_sorcery(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if let Some(SolarSorcererView::Solar(solar)) = &mut self.sorcery {
            self.sorcery = Some(SolarSorcererView::Celestial((&*solar).into()));
            Ok(self)
        } else {
            Err(CharacterMutationError::SorceryError(
                SorceryError::CircleSequence,
            ))
        }
    }

    pub(crate) fn add_sorcery_archetype_merit(
        &mut self,
        sorcery_archetype_name: &str,
        sorcery_archetype_merit_name: &'source str,
        sorcery_archetype_merit: &'source SorceryArchetypeMeritDetails,
    ) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.sorcery {
            Some(SolarSorcererView::Terrestrial(terrestrial)) => {
                if terrestrial.archetype_name != sorcery_archetype_name {
                    Err(CharacterMutationError::SorceryError(
                        SorceryError::MissingArchetype,
                    ))
                } else if let Entry::Vacant(e) = terrestrial
                    .archetype_merits
                    .entry(sorcery_archetype_merit_name)
                {
                    e.insert(sorcery_archetype_merit);
                    Ok(self)
                } else {
                    Err(CharacterMutationError::MeritError(
                        MeritError::DuplicateMerit,
                    ))
                }
            }
            Some(SolarSorcererView::Celestial(celestial)) => {
                if let Some((_, merits)) = celestial.archetypes.get_mut(sorcery_archetype_name) {
                    if let Entry::Vacant(e) = merits.entry(sorcery_archetype_merit_name) {
                        e.insert(sorcery_archetype_merit);
                        Ok(self)
                    } else {
                        Err(CharacterMutationError::MeritError(
                            MeritError::DuplicateMerit,
                        ))
                    }
                } else {
                    Err(CharacterMutationError::SorceryError(
                        SorceryError::MissingArchetype,
                    ))
                }
            }
            Some(SolarSorcererView::Solar(solar)) => {
                if let Some((_, merits)) = solar.archetypes.get_mut(sorcery_archetype_name) {
                    if let Entry::Vacant(e) = merits.entry(sorcery_archetype_merit_name) {
                        e.insert(sorcery_archetype_merit);
                        Ok(self)
                    } else {
                        Err(CharacterMutationError::MeritError(
                            MeritError::DuplicateMerit,
                        ))
                    }
                } else {
                    Err(CharacterMutationError::SorceryError(
                        SorceryError::MissingArchetype,
                    ))
                }
            }
            None => Err(CharacterMutationError::SorceryError(
                SorceryError::MissingArchetype,
            )),
        }
    }

    pub(crate) fn remove_sorcery_archetype_merit(
        &mut self,
        name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.sorcery {
            Some(SolarSorcererView::Terrestrial(terrestrial)) => {
                if terrestrial
                    .archetype_merits
                    .remove(name)
                    .is_none()
                {
                    Err(CharacterMutationError::MeritError(MeritError::NotFound))
                } else {
                    Ok(self)
                }
            }
            Some(SolarSorcererView::Celestial(celestial)) => {
                if !celestial
                    .archetypes
                    .iter_mut()
                    .any(|(_, (_, merits))| merits.remove(name).is_some())
                {
                    Err(CharacterMutationError::MeritError(MeritError::NotFound))
                } else {
                    Ok(self)
                }
            }
            Some(SolarSorcererView::Solar(solar)) => {
                if !solar
                    .archetypes
                    .iter_mut()
                    .any(|(_, (_, merits))| merits.remove(name).is_some())
                {
                    Err(CharacterMutationError::MeritError(MeritError::NotFound))
                } else {
                    Ok(self)
                }
            }
            None => Err(CharacterMutationError::MeritError(MeritError::NotFound)),
        }
    }

    pub(crate) fn correct_sorcery_level(&mut self, occult_dots: u8, essence_rating: u8) -> bool {
        let mut removal_happened = false;

        if (occult_dots < 5 || essence_rating < 5)
            && matches!(self.sorcery, Some(SolarSorcererView::Solar(_)))
        {
            let solar_removed = self.remove_solar_sorcery().is_ok();
            removal_happened = solar_removed || removal_happened;
        }

        if (occult_dots < 4 || essence_rating < 3)
            && matches!(self.sorcery, Some(SolarSorcererView::Celestial(_)))
        {
            let celestial_removed = self.remove_celestial_sorcery().is_ok();
            removal_happened = celestial_removed || removal_happened;
        }

        if occult_dots < 3 && self.sorcery.is_some() {
            let terrestrial_removed = self.remove_terrestrial_sorcery().is_ok();
            removal_happened = terrestrial_removed || removal_happened;
        }

        removal_happened
    }

    pub(crate) fn add_solar_charm(
        &mut self,
        name: &'source str,
        charm: &'source SolarCharm,
        ability_dots: u8,
        essence_rating: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if charm.ability_requirement().1 > ability_dots
            || (Into::<AbilityName>::into(charm.ability_requirement().0) != self.supernal_ability()
                && charm.essence_required().get() > essence_rating)
        {
            return Err(CharacterMutationError::CharmError(
                CharmError::PrerequisitesNotMet,
            ));
        }
        let mut unmet_tree_requirements = charm.charm_prerequisites().collect::<HashSet<&str>>();

        for known_charm_name in self
            .solar_charms
            .iter()
            .map(|(known_charm_name, _)| known_charm_name)
        {
            if known_charm_name == &name {
                return Err(CharacterMutationError::CharmError(
                    CharmError::DuplicateCharm,
                ));
            } else {
                unmet_tree_requirements.remove(known_charm_name);
            }
        }

        if !unmet_tree_requirements.is_empty() {
            return Err(CharacterMutationError::CharmError(
                CharmError::PrerequisitesNotMet,
            ));
        }

        self.solar_charms.push((name, charm));
        Ok(self)
    }

    pub(crate) fn get_solar_charm(&self, name: &str) -> Option<Charm<'source>> {
        self.solar_charms
            .iter()
            .find_map(|(solar_charm_name, charm)| {
                if solar_charm_name == &name {
                    Some(Charm::Solar(charm))
                } else {
                    None
                }
            })
    }

    pub(crate) fn add_spell(
        &mut self,
        name: &'source str,
        spell: &'source SpellMutation,
    ) -> Result<&mut Self, CharacterMutationError> {
        match (&mut self.sorcery, spell) {
            (None, _) => Err(CharacterMutationError::CharmError(
                CharmError::PrerequisitesNotMet,
            )),
            (
                Some(SolarSorcererView::Terrestrial(terrestrial)),
                SpellMutation::Terrestrial(terrestrial_spell),
            ) => {
                terrestrial.add_terrestrial_spell(name, terrestrial_spell)?;
                Ok(self)
            }
            (Some(SolarSorcererView::Terrestrial(_)), _intelligence_dots) => Err(
                CharacterMutationError::CharmError(CharmError::PrerequisitesNotMet),
            ),
            (
                Some(SolarSorcererView::Celestial(celestial)),
                SpellMutation::Terrestrial(terrestrial_spell),
            ) => {
                celestial.add_terrestrial_spell(name, terrestrial_spell)?;
                Ok(self)
            }
            (
                Some(SolarSorcererView::Celestial(celestial)),
                SpellMutation::Celestial(celestial_spell),
            ) => {
                celestial.add_celestial_spell(name, celestial_spell)?;
                Ok(self)
            }
            (Some(SolarSorcererView::Celestial(_)), SpellMutation::Solar(_)) => Err(
                CharacterMutationError::CharmError(CharmError::PrerequisitesNotMet),
            ),
            (Some(SolarSorcererView::Solar(solar)), spell_mutation) => {
                solar.add_spell(name, spell_mutation)?;
                Ok(self)
            }
        }
    }

    pub(crate) fn remove_spell(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.sorcery {
            Some(SolarSorcererView::Terrestrial(terrestrial)) => {
                terrestrial.remove_spell(name)?;
            }
            Some(SolarSorcererView::Celestial(celestial)) => {
                celestial.remove_spell(name)?;
            }
            Some(SolarSorcererView::Solar(solar)) => {
                solar.remove_spell(name)?;
            }
            None => {
                return Err(CharacterMutationError::CharmError(CharmError::NotFound));
            }
        }
        Ok(self)
    }

    pub(crate) fn get_eclipse_charm(&self, name: &str) -> Option<Charm<'source>> {
        match &self.caste {
            SolarCaste::Eclipse(eclipse) => eclipse
                .eclipse_charms
                .get(&name)
                .map(|eclipse_charm| Charm::Eclipse(eclipse_charm)),
            _ => None,
        }
    }

    pub(crate) fn eclipse_charms_iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        match &self.caste {
            SolarCaste::Eclipse(eclipse) => eclipse
                .eclipse_charms
                .keys()
                .copied()
                .collect::<Vec<&str>>(),
            _ => vec![],
        }
        .into_iter()
    }
}

impl<'view, 'source> Solar<'source> {
    pub(crate) fn sorcery(&'view self) -> Option<&'view SolarSorcererView<'source>> {
        self.sorcery.as_ref()
    }
}
