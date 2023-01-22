/// Traits relating to specific Solar castes.
pub mod caste;

/// A builder path for constructing a new Solar.
pub mod builder;

/// Details of a Solar Charm.
pub mod charm;

mod error;
mod memo;
mod new;
mod sorcery;

use std::collections::{hash_map::Entry, HashSet};

pub use error::SolarError;
pub(crate) use memo::SolarMemo;
pub use new::NewSolar;
pub(crate) use sorcery::{SolarSorcererMemo, SolarSorcererView};

use crate::{
    abilities::AbilityName,
    exaltation::exalt::Limit,
    merits::merit::MeritError,
    sorcery::{
        circles::{
            celestial::AddCelestialSorcery,
            solar::AddSolarSorcery,
            terrestrial::{sorcerer::TerrestrialCircleSorcerer, AddTerrestrialSorceryView},
        },
        SorceryArchetypeId, SorceryArchetypeMerit, SorceryArchetypeMeritId, SorceryError,
    },
    CharacterMutationError, charms::{CharmError, charm::Charm},
};

use self::{builder::SolarBuilder, caste::SolarCaste, charm::{SolarCharmId, SolarCharm}};

/// Traits which are unique to being a Solar Exalted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Solar<'source> {
    caste: SolarCaste,
    favored_abilities: [AbilityName; 5],
    pub(crate) sorcery: Option<SolarSorcererView<'source>>,
    limit: Limit<'source>,
    pub(crate) solar_charms: Vec<(SolarCharmId, &'source SolarCharm)>,
}

impl<'source> Solar<'source> {
    /// Starts building a set of Solar traits
    pub fn builder() -> SolarBuilder {
        SolarBuilder {
            limit_trigger: None,
        }
    }

    pub(crate) fn as_memo(&self) -> SolarMemo {
        SolarMemo {
            caste: self.caste.as_memo(),
            favored_abilities: self.favored_abilities,
            sorcery: self.sorcery.as_ref().map(|sorcery| sorcery.as_memo()),
            limit: self.limit.as_memo(),
            solar_charms: self.solar_charms.iter().map(|(charm_id, charm)| (*charm_id, (*charm).to_owned())).collect(),
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

    pub(crate) fn add_terrestrial_sorcery(
        &mut self,
        add_terrestrial: AddTerrestrialSorceryView<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.sorcery.is_none() {
            self.sorcery = Some(SolarSorcererView::Terrestrial(
                TerrestrialCircleSorcerer::new(
                    add_terrestrial.archetype_id,
                    add_terrestrial.archetype,
                    add_terrestrial.shaping_ritual_id,
                    add_terrestrial.shaping_ritual,
                    add_terrestrial.control_spell_id,
                    add_terrestrial.control_spell,
                )?,
            ));
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
            self.sorcery = Some(SolarSorcererView::Terrestrial((&*celestial).into()));
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
        sorcery_archetype_id: SorceryArchetypeId,
        sorcery_archetype_merit_id: SorceryArchetypeMeritId,
        sorcery_archetype_merit: &'source SorceryArchetypeMerit,
    ) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.sorcery {
            Some(SolarSorcererView::Terrestrial(terrestrial)) => {
                if terrestrial.archetype_id != sorcery_archetype_id {
                    Err(CharacterMutationError::SorceryError(
                        SorceryError::MissingArchetype,
                    ))
                } else if let Entry::Vacant(e) = terrestrial
                    .archetype_merits
                    .entry(sorcery_archetype_merit_id)
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
                if let Some((_, merits)) = celestial.archetypes.get_mut(&sorcery_archetype_id) {
                    if let Entry::Vacant(e) = merits.entry(sorcery_archetype_merit_id) {
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
                if let Some((_, merits)) = solar.archetypes.get_mut(&sorcery_archetype_id) {
                    if let Entry::Vacant(e) = merits.entry(sorcery_archetype_merit_id) {
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
        sorcery_archetype_merit_id: SorceryArchetypeMeritId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.sorcery {
            Some(SolarSorcererView::Terrestrial(terrestrial)) => {
                if terrestrial
                    .archetype_merits
                    .remove(&sorcery_archetype_merit_id)
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
                    .any(|(_, (_, merits))| merits.remove(&sorcery_archetype_merit_id).is_some())
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
                    .any(|(_, (_, merits))| merits.remove(&sorcery_archetype_merit_id).is_some())
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
            removal_happened =  solar_removed || removal_happened;
        }

        if (occult_dots < 4 || essence_rating < 3)
            && matches!(self.sorcery, Some(SolarSorcererView::Celestial(_)))
        {
            let celestial_removed = self.remove_celestial_sorcery().is_ok();
            removal_happened =  celestial_removed || removal_happened;
        }

        if occult_dots < 3 && self.sorcery.is_some() {
            let terrestrial_removed = self.remove_terrestrial_sorcery().is_ok();
            removal_happened =  terrestrial_removed || removal_happened;
        }

        removal_happened
    }

    pub(crate) fn add_solar_charm(&mut self, charm_id: SolarCharmId, charm: &'source SolarCharm, ability_dots: u8, essence_rating: u8) -> Result<&mut Self, CharacterMutationError> {
        if charm.ability_requirement().1 > ability_dots || (Into::<AbilityName>::into( charm.ability_requirement().0) != self.supernal_ability() && charm.essence_required().get() > essence_rating) {
            return Err(CharacterMutationError::CharmError(CharmError::PrerequisitesNotMet));
        } 
        let mut unmet_tree_requirements = charm.charm_prerequisites().collect::<HashSet<SolarCharmId>>();

        for known_charm_id in self.solar_charms.iter().map(|(known_charm_id, _)| known_charm_id) {
            if known_charm_id == &charm_id {
                return Err(CharacterMutationError::CharmError(CharmError::DuplicateCharm));
            } else {
                unmet_tree_requirements.remove(known_charm_id);
            }
        }

        if !unmet_tree_requirements.is_empty() {
            return Err(CharacterMutationError::CharmError(CharmError::PrerequisitesNotMet));
        }

        self.solar_charms.push((charm_id, charm));
        Ok(self)
    }

    pub(crate) fn get_solar_charm(&self, charm_id: SolarCharmId) -> Option<Charm<'source>> {
        self.solar_charms.iter().find_map(|(solar_charm_id, charm)| if solar_charm_id == &charm_id {
            Some(Charm::Solar(charm))
        } else {
            None
        })
    }
}

impl<'view, 'source> Solar<'source> {
    pub(crate) fn sorcery(&'view self) -> Option<&'view SolarSorcererView<'source>> {
        self.sorcery.as_ref()
    }
}
