use std::collections::HashSet;

use crate::{
    armor::{
        armor_item::{artifact::ArtifactError, ArmorId},
        ArmorError,
    },
    artifact::ArtifactId,
    charms::{
        charm::{
            evocation::{Evocation, EvocationId, EvokableId},
            Charm, CharmId,
        },
        CharmError,
    },
    exaltation::Exaltation,
    hearthstones::HearthstoneError,
    weapons::{weapon::WeaponId, WeaponError},
    Character, CharacterMutationError,
};

impl<'source> Character<'source> {
    /// Adds an evocation to the character.
    pub fn add_evocation(
        &mut self,
        evocation_id: EvocationId,
        evocation: &'source Evocation,
    ) -> Result<&mut Self, CharacterMutationError> {
        match evocation.evokable_id() {
            EvokableId::Hearthstone(hearthstone_id) => {
                if self.hearthstones().get(hearthstone_id).is_none() {
                    return Err(CharacterMutationError::HearthstoneError(
                        HearthstoneError::NotFound,
                    ));
                }
            }
            EvokableId::Artifact(ArtifactId::Armor(artifact_armor_id)) => {
                if self
                    .armor()
                    .get(ArmorId::Artifact(artifact_armor_id))
                    .is_none()
                {
                    return Err(CharacterMutationError::ArmorError(ArmorError::NotFound));
                }
            }
            EvokableId::Artifact(ArtifactId::Weapon(artifact_weapon_id)) => {
                if !self
                    .weapons()
                    .iter()
                    .any(|(weapon_id, _)| weapon_id == WeaponId::Artifact(artifact_weapon_id))
                {
                    return Err(CharacterMutationError::WeaponError(WeaponError::NotFound));
                }
            }
            EvokableId::Artifact(ArtifactId::Wonder(wonder_id)) => {
                if self.wonders().get(wonder_id).is_none() {
                    return Err(CharacterMutationError::ArtifactError(
                        ArtifactError::NotFound,
                    ));
                }
            }
        };

        self.exaltation.add_evocation(evocation_id, evocation)?;
        Ok(self)
    }

    pub(crate) fn correct_evocations(&mut self, force_remove: &[EvocationId]) -> bool {
        let actual_essence = if let Some(essence) = self.essence() {
            essence.rating()
        } else {
            return false;
        };

        let remove_ids: HashSet<EvocationId> = self
            .charms()
            .iter()
            .filter_map(|charm_id| {
                if let CharmId::Evocation(known_evocation_id) = charm_id {
                    self.charms().get(charm_id).and_then(|charm| {
                        if let Charm::Evocation(evocation) = charm {
                            Some((known_evocation_id, evocation))
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            })
            .fold(
                HashSet::from_iter(force_remove.iter().copied()),
                |mut ids_to_remove, (evocation_id, evocation)| {
                    match evocation.evokable_id() {
                        EvokableId::Hearthstone(hearthstone_id) => {
                            if self.hearthstones().get(hearthstone_id).is_none() {
                                ids_to_remove.insert(evocation_id);
                            }
                        }
                        EvokableId::Artifact(ArtifactId::Armor(artifact_armor_id)) => {
                            if self
                                .armor()
                                .get(ArmorId::Artifact(artifact_armor_id))
                                .is_none()
                            {
                                ids_to_remove.insert(evocation_id);
                            }
                        }
                        EvokableId::Artifact(ArtifactId::Weapon(artifact_weapon_id)) => {
                            if !self.weapons().iter().any(|(weapon_id, _)| {
                                weapon_id == WeaponId::Artifact(artifact_weapon_id)
                            }) {
                                ids_to_remove.insert(evocation_id);
                            }
                        }
                        EvokableId::Artifact(ArtifactId::Wonder(wonder_id)) => {
                            if self.wonders().get(wonder_id).is_none() {
                                ids_to_remove.insert(evocation_id);
                            }
                        }
                    };

                    if evocation.essence_required() > actual_essence {
                        ids_to_remove.insert(evocation_id);
                    }

                    for prerequisite_evocation_id in evocation.evocation_prerequisites() {
                        if ids_to_remove.contains(&prerequisite_evocation_id) {
                            ids_to_remove.insert(evocation_id);
                            break;
                        }
                    }

                    if let Some(charm_id) = evocation.upgrade() {
                        if let CharmId::Evocation(upgrade_evocation_id) = charm_id {
                            if ids_to_remove.contains(&upgrade_evocation_id) {
                                ids_to_remove.insert(evocation_id);
                            }
                        }

                        if self.charms().get(charm_id).is_none() {
                            ids_to_remove.insert(evocation_id);
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
            exalt.evocations.retain(|(id, _)| !remove_ids.contains(id));
            exalt.evocations.len() < old_len
        } else {
            false
        }
    }

    /// Removes an evocation from the character.
    pub fn remove_evocation(
        &mut self,
        evocation_id: EvocationId,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.correct_evocations(&[evocation_id]) {
            Ok(self)
        } else {
            Err(CharacterMutationError::CharmError(CharmError::NotFound))
        }
    }
}
