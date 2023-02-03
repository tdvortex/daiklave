use std::collections::HashSet;

use crate::{
    armor::{
        armor_item::{artifact::ArtifactError, ArmorName},
        ArmorError,
    },
    artifact::ArtifactName,
    charms::{
        charm::{
            evocation::{EvokableName, AddEvocation},
            Charm, CharmName,
        },
        CharmError,
    },
    exaltation::Exaltation,
    hearthstones::HearthstoneError,
    weapons::{weapon::WeaponName, WeaponError},
    Character, CharacterMutationError,
};

impl<'source> Character<'source> {
    /// Adds an evocation to the character.
    pub fn add_evocation(
        &mut self,
        add_evocation: &'source AddEvocation
    ) -> Result<&mut Self, CharacterMutationError> {
        let AddEvocation {
            name,
            evocation,
        } = add_evocation;
        match evocation.evokable_name() {
            EvokableName::Hearthstone(hearthstone_id) => {
                if self.hearthstones().get(hearthstone_id).is_none() {
                    return Err(CharacterMutationError::HearthstoneError(
                        HearthstoneError::NotFound,
                    ));
                }
            }
            EvokableName::Artifact(ArtifactName::Armor(name)) => {
                if self.armor().get(ArmorName::Artifact(name)).is_none() {
                    return Err(CharacterMutationError::ArmorError(ArmorError::NotFound));
                }
            }
            EvokableName::Artifact(ArtifactName::Weapon(name)) => {
                if !self.weapons().iter().any(|(weapon_name, _)| {
                    if let WeaponName::Artifact(actual_name) = weapon_name {
                        actual_name == name
                    } else {
                        false
                    }
                }) {
                    return Err(CharacterMutationError::WeaponError(WeaponError::NotFound));
                }
            }
            EvokableName::Artifact(ArtifactName::Wonder(name)) => {
                if self.wonders().get(name).is_none() {
                    return Err(CharacterMutationError::ArtifactError(
                        ArtifactError::NotFound,
                    ));
                }
            }
        };

        self.exaltation.add_evocation(name, evocation)?;
        Ok(self)
    }

    pub(crate) fn correct_evocations(&mut self, force_remove: &[&str]) -> bool {
        let actual_essence = if let Some(essence) = self.essence() {
            essence.rating()
        } else {
            return false;
        };

        let charms = self.charms();

        let remove_ids: HashSet<String> = charms
            .iter()
            .filter_map(|charm_id| {
                if let CharmName::Evocation(known_evocation_name) = charm_id {
                    charms.get(charm_id).and_then(|charm| {
                        if let Charm::Evocation(evocation) = charm {
                            Some((known_evocation_name, evocation))
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            })
            .fold(
                HashSet::from_iter(force_remove.iter().map(|&s| s.to_owned())),
                |mut ids_to_remove, (evocation_name, evocation)| {
                    match evocation.evokable_name() {
                        EvokableName::Hearthstone(name) => {
                            if self.hearthstones().get(name).is_none() {
                                ids_to_remove.insert(evocation_name.to_owned());
                            }
                        }
                        EvokableName::Artifact(ArtifactName::Armor(name)) => {
                            if self.armor().get(ArmorName::Artifact(name)).is_none() {
                                ids_to_remove.insert(evocation_name.to_owned());
                            }
                        }
                        EvokableName::Artifact(ArtifactName::Weapon(name)) => {
                            if !self.weapons().iter().any(|(weapon_name, _)| {
                                if let WeaponName::Artifact(artifact_name) = weapon_name {
                                    artifact_name == name
                                } else {
                                    false
                                }
                            }) {
                                ids_to_remove.insert(evocation_name.to_owned());
                            }
                        }
                        EvokableName::Artifact(ArtifactName::Wonder(name)) => {
                            if self.wonders().get(name).is_none() {
                                ids_to_remove.insert(evocation_name.to_owned());
                            }
                        }
                    };

                    if evocation.essence_required() > actual_essence {
                        ids_to_remove.insert(evocation_name.to_owned());
                    }

                    for prerequisite_evocation_name in evocation.evocation_prerequisites() {
                        if ids_to_remove.contains(prerequisite_evocation_name) {
                            ids_to_remove.insert(evocation_name.to_owned());
                            break;
                        }
                    }

                    if let Some(charm_id) = evocation.upgrade() {
                        if let CharmName::Evocation(upgrade_evocation_id) = charm_id {
                            if ids_to_remove.contains(upgrade_evocation_id) {
                                ids_to_remove.insert(evocation_name.to_owned());
                            }
                        }

                        if self.charms().get(charm_id).is_none() {
                            ids_to_remove.insert(evocation_name.to_owned());
                        }
                    }

                    ids_to_remove
                },
            );

        if remove_ids.is_empty() {
            return false;
        }

        if let Exaltation::Exalt(exalt) = &mut self.exaltation {
            let old_len = exalt.evocations.len();
            exalt.evocations.retain(|(id, _)| !remove_ids.contains(*id));
            exalt.evocations.len() < old_len
        } else {
            false
        }
    }

    /// Removes an evocation from the character.
    pub fn remove_evocation(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        if self.correct_evocations(&[name]) {
            Ok(self)
        } else {
            Err(CharacterMutationError::CharmError(CharmError::NotFound))
        }
    }
}
