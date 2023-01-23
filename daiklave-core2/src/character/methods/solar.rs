use std::collections::{hash_map::Entry, HashSet};

use crate::{
    abilities::AbilityName,
    charms::{
        charm::{Charm, CharmId, EclipseCharm, SpiritCharmId},
        CharmError,
    },
    exaltation::{
        exalt::exalt_type::{
            solar::{
                caste::SolarCaste,
                charm::{SolarCharm, SolarCharmAbility, SolarCharmId},
                NewSolar, Solar,
            },
            ExaltType,
        },
        Exaltation,
    },
    Character, CharacterMutationError,
};

impl<'source> Character<'source> {
    /// Returns true if character is a Solar.
    pub fn is_solar(&self) -> bool {
        self.exaltation.is_solar()
    }

    /// Returns the character's Solar-specific traits, or None if not a Solar.
    pub fn solar_traits(&'source self) -> Option<&Solar> {
        self.exaltation.solar_traits()
    }

    /// Sets a character's Exaltation to be the given Solar exaltation. If the
    /// character was previously mortal, permanent willpower rating is
    /// increased by 2 (reflecting the difference between mortal default and
    /// Exalt default).
    pub fn set_solar(
        &mut self,
        solar: &'source NewSolar,
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
        solar_charm_id: SolarCharmId,
        charm: &'source SolarCharm,
    ) -> Result<&mut Self, CharacterMutationError> {
        let ability_dots = match charm.ability_requirement() {
            (SolarCharmAbility::Craft, _) => self.craft().max(),
            (solar_ability, _) => self
                .abilities()
                .get(solar_ability.try_into().unwrap())
                .dots(),
        };

        self.exaltation
            .add_solar_charm(solar_charm_id, charm, ability_dots)?;
        Ok(self)
    }

    /// Removes a Solar Charm from the character.
    pub fn remove_solar_charm(
        &mut self,
        solar_charm_id: SolarCharmId,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.correct_solar_charms(&[solar_charm_id]) {
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
        charm_id: SpiritCharmId,
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
                } else if let Entry::Vacant(e) = eclipse.eclipse_charms.entry(charm_id) {
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

    pub(crate) fn correct_eclipse_charms(&mut self, force_remove: &[SpiritCharmId]) -> bool {
        if let Exaltation::Exalt(exalt) = &mut self.exaltation {
            let actual_essence = exalt.essence.rating;

            let ExaltType::Solar(solar) = &mut exalt.exalt_type;
            if let SolarCaste::Eclipse(eclipse) = &mut solar.caste {
                let mut ids_to_remove: HashSet<SpiritCharmId> =
                    HashSet::from_iter(force_remove.iter().copied());
                for (id, charm) in eclipse.eclipse_charms.iter() {
                    if charm.essence_required() > actual_essence.get() {
                        ids_to_remove.insert(*id);
                    }
                }

                if !ids_to_remove.is_empty() {
                    let old_len = eclipse.eclipse_charms.len();
                    for id in ids_to_remove.into_iter() {
                        eclipse.eclipse_charms.remove(&id);
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
        charm_id: SpiritCharmId,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.correct_eclipse_charms(&[charm_id]) {
            // May lose evocations which upgrade the solar charm
            self.correct_evocations(&[]);
            Ok(self)
        } else {
            Err(CharacterMutationError::CharmError(CharmError::NotFound))
        }
    }

    pub(crate) fn solar_charms_iter(&self) -> impl Iterator<Item = SolarCharmId> + '_ {
        self.exaltation.solar_charms_iter()
    }

    pub(crate) fn correct_solar_charms(&mut self, force_remove: &[SolarCharmId]) -> bool {
        let actual_essence = if let Some(essence) = self.essence() {
            essence.rating()
        } else {
            return false;
        };

        let ids_to_remove = self
            .charms()
            .iter()
            .filter(|charm_id| matches!(charm_id, CharmId::Solar(_)))
            .flat_map(|charm_id| {
                self.charms()
                    .get(charm_id)
                    .and_then(|charm| match (charm_id, charm) {
                        (CharmId::Solar(solar_charm_id), Charm::Solar(solar_charm)) => {
                            Some((solar_charm_id, solar_charm))
                        }
                        _ => None,
                    })
                    .into_iter()
            })
            .fold(
                HashSet::<SolarCharmId>::from_iter(force_remove.iter().copied()),
                |mut ids_to_remove, (charm_id, charm)| {
                    if charm
                        .charm_prerequisites()
                        .any(|prereq_id| ids_to_remove.contains(&prereq_id))
                    {
                        ids_to_remove.insert(charm_id);
                    }

                    let (ability_name, dots_required) = charm.ability_requirement();
                    let actual_dots = match ability_name {
                        SolarCharmAbility::Craft => self.craft().max(),
                        other_solar_ability => self
                            .abilities()
                            .get(other_solar_ability.try_into().unwrap())
                            .dots(),
                    };
                    if dots_required > actual_dots {
                        ids_to_remove.insert(charm_id);
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
                            ids_to_remove.insert(charm_id);
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
                .retain(|(charm_id, _charm)| !ids_to_remove.contains(charm_id));
            solar.solar_charms.len() < old_size
        } else {
            false
        }
    }
}
