use std::collections::{hash_map::Entry, HashSet};

use crate::{
    abilities::AbilityName,
    charms::{
        charm::{Charm, CharmName, EclipseCharm},
        CharmError,
    },
    exaltation::{
        exalt::exalt_type::{
            solar::{
                caste::SolarCaste,
                charm::{AddSolarCharm, SolarCharmAbility},
                SetSolar,
            },
            ExaltType,
        },
        Exaltation,
    },
    Character, CharacterMutationError,
};

impl<'source> Character<'source> {
    /// Sets a character's Exaltation to be the given Solar exaltation. If the
    /// character was previously mortal, permanent willpower rating is
    /// increased by 2 (reflecting the difference between mortal default and
    /// Exalt default).
    pub fn set_solar(
        &mut self,
        solar: &'source SetSolar,
    ) -> Result<&mut Self, CharacterMutationError> {
        let new_willpower_rating = self
            .willpower()
            .rating()
            .saturating_add(2 * u8::from(self.is_mortal()));
        self.exaltation.set_solar(solar.0.as_ref())?;
        self.set_willpower_rating(new_willpower_rating)?;

        self.correct_merits();
        self.correct_martial_arts_charms(&[]);
        self.correct_solar_charms(&[]);
        self.correct_evocations(&[]);
        Ok(self)
    }

    /// Adds a Solar Charm to the character.
    pub fn add_solar_charm(
        &mut self,
        add_solar_charm: &'source AddSolarCharm,
    ) -> Result<&mut Self, CharacterMutationError> {
        let ability_dots = match add_solar_charm.details.ability {
            SolarCharmAbility::Craft => self.craft().max(),
            solar_ability => self
                .abilities()
                .get_vanilla(solar_ability.try_into().unwrap())
                .dots(),
        };

        self.exaltation.add_solar_charm(
            &add_solar_charm.name,
            &add_solar_charm.details,
            ability_dots,
        )?;
        Ok(self)
    }

    /// Removes a Solar Charm from the character.
    pub fn remove_solar_charm(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        if self.correct_solar_charms(&[name]) {
            // May lose evocations which upgrade the solar charm
            self.correct_evocations(&[]);
            Ok(self)
        } else {
            Err(CharacterMutationError::CharmError(CharmError::NotFound))
        }
    }

    /// Adds an Eclipse Charm to the character.
    pub fn add_eclipse_charm(
        &mut self,
        name: &'source str,
        charm: &'source EclipseCharm,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Exaltation::Exalt(exalt) = &mut self.exaltation {
            let actual_essence = exalt.essence.rating;

            let ExaltType::Solar(solar) = &mut exalt.exalt_type;
            if let SolarCaste::Eclipse(eclipse) = &mut solar.caste {
                if charm.essence_required() > actual_essence.get() {
                    Err(CharacterMutationError::CharmError(
                        CharmError::PrerequisitesNotMet,
                    ))
                } else if let Entry::Vacant(e) = eclipse.eclipse_charms.entry(name) {
                    e.insert(charm);
                    Ok(self)
                } else {
                    Err(CharacterMutationError::CharmError(
                        CharmError::DuplicateCharm,
                    ))
                }
            } else {
                Err(CharacterMutationError::CharmError(
                    CharmError::WrongExaltType,
                ))
            }
        } else {
            Err(CharacterMutationError::CharmError(CharmError::Mortal))
        }
    }

    pub(crate) fn correct_eclipse_charms(&mut self, force_remove: &[&str]) -> bool {
        if let Exaltation::Exalt(exalt) = &mut self.exaltation {
            let actual_essence = exalt.essence.rating;

            let ExaltType::Solar(solar) = &mut exalt.exalt_type;
            if let SolarCaste::Eclipse(eclipse) = &mut solar.caste {
                let mut charms_to_remove: HashSet<String> =
                    HashSet::from_iter(force_remove.iter().map(|s| (*s).to_owned()));
                for (name, charm) in eclipse.eclipse_charms.iter() {
                    if charm.essence_required() > actual_essence.get() {
                        charms_to_remove.insert((*name).to_owned());
                    }
                }

                if !charms_to_remove.is_empty() {
                    let old_len = eclipse.eclipse_charms.len();
                    for name in charms_to_remove.into_iter() {
                        eclipse.eclipse_charms.remove(name.as_str());
                    }
                    return old_len > eclipse.eclipse_charms.len();
                }
            }
        }

        false
    }

    /// Removes an Eclipse Charm from the character.
    pub fn remove_eclipse_charm(
        &mut self,
        name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.correct_eclipse_charms(&[name]) {
            // May lose evocations which upgrade the eclipse charm
            self.correct_evocations(&[]);
            Ok(self)
        } else {
            Err(CharacterMutationError::CharmError(CharmError::NotFound))
        }
    }

    pub(crate) fn solar_charms_iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        self.exaltation.solar_charms_iter()
    }

    pub(crate) fn correct_solar_charms(&mut self, force_remove: &[&str]) -> bool {
        let actual_essence = if let Some(essence) = self.essence() {
            essence.rating()
        } else {
            return false;
        };

        let ids_to_remove = self
            .charms()
            .iter()
            .filter(|charm_id| matches!(charm_id, CharmName::Solar(_)))
            .flat_map(|charm_id| {
                self.charms()
                    .get(charm_id)
                    .and_then(|charm| match (charm_id, charm) {
                        (CharmName::Solar(solar_charm_id), Charm::Solar(solar_charm)) => {
                            Some((solar_charm_id, solar_charm))
                        }
                        _ => None,
                    })
                    .into_iter()
            })
            .fold(
                HashSet::<String>::from_iter(force_remove.iter().map(|&s| s.to_owned())),
                |mut ids_to_remove, (charm_name, charm)| {
                    if charm
                        .charm_prerequisites()
                        .any(|prereq_name| ids_to_remove.contains(prereq_name))
                    {
                        ids_to_remove.insert(charm_name.to_owned());
                    }

                    let (ability_name, dots_required) = charm.ability_requirement();
                    let actual_dots = match ability_name {
                        SolarCharmAbility::Craft => self.craft().max(),
                        other_solar_ability => self
                            .abilities()
                            .get_vanilla(other_solar_ability.try_into().unwrap())
                            .dots(),
                    };
                    if dots_required > actual_dots {
                        ids_to_remove.insert(charm_name.to_owned());
                    }

                    let essence_required = charm.essence_required().get();
                    if essence_required > actual_essence {
                        let mut is_supernal = false;
                        if let Exaltation::Exalt(exalt) = &self.exaltation {
                            let ExaltType::Solar(solar) = &exalt.as_ref().exalt_type;
                            if Into::<AbilityName>::into(ability_name) == solar.supernal_ability() {
                                is_supernal = true;
                            }
                        }
                        if !is_supernal {
                            ids_to_remove.insert(charm_name.to_owned());
                        }
                    }

                    ids_to_remove
                },
            );

        if let Exaltation::Exalt(exalt) = &mut self.exaltation {
            let ExaltType::Solar(solar) = &mut exalt.as_mut().exalt_type;
            let old_size = solar.solar_charms.len();
            solar
                .solar_charms
                .retain(|(charm_name, _charm)| !ids_to_remove.contains(*charm_name));
            solar.solar_charms.len() < old_size
        } else {
            false
        }
    }
}
