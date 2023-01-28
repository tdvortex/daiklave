use std::collections::hash_map::Entry;

use crate::{
    artifact::ArtifactName,
    hearthstones::{
        hearthstone::HearthstoneTemplate, HearthstoneError, HearthstoneOrigin,
        HearthstoneStability, Hearthstones, UnslottedHearthstone,
    },
    Character, CharacterMutationError,
};

impl<'view, 'source> Character<'source> {
    /// Gets the character's Hearthstones.
    pub fn hearthstones(&'view self) -> Hearthstones<'view, 'source> {
        Hearthstones(self)
    }

    /// Adds a manse (and demense and hearthstone) to the character.
    pub fn add_manse(
        &mut self,
        manse_name: &'source str,
        demense_name: &'source str,
        hearthstone_name: &'source str,
        template: &'source HearthstoneTemplate,
    ) -> Result<&mut Self, CharacterMutationError> {
        let unslotted = UnslottedHearthstone {
            details: template.details.as_ref(),
            origin: match template.stability {
                HearthstoneStability::Linked => {
                    HearthstoneOrigin::Linked((manse_name, demense_name))
                }
                HearthstoneStability::ManseBorn => {
                    HearthstoneOrigin::ManseBorn(Some((manse_name, demense_name)))
                }
                HearthstoneStability::ManseBornSteady => {
                    HearthstoneOrigin::ManseBornSteady(Some((manse_name, demense_name)))
                }
                HearthstoneStability::Steady => {
                    HearthstoneOrigin::Steady(Some((manse_name, demense_name)))
                }
                HearthstoneStability::WildBorn => {
                    return Err(CharacterMutationError::HearthstoneError(
                        HearthstoneError::WildBornWithManse,
                    ));
                }
                HearthstoneStability::Unspecified => {
                    HearthstoneOrigin::Unspecified(Some((manse_name, demense_name)))
                }
            },
        };
        self.hearthstone_inventory
            .insert(hearthstone_name, unslotted);
        Ok(self)
    }

    /// Adds a standalone hearthstone (without a manse) to the character.
    pub fn add_hearthstone(
        &mut self,
        hearthstone_name: &'source str,
        template: &'source HearthstoneTemplate,
    ) -> Result<&mut Self, CharacterMutationError> {
        let unslotted = UnslottedHearthstone {
            details: template.details.as_ref(),
            origin: match template.stability {
                HearthstoneStability::Linked => {
                    return Err(CharacterMutationError::HearthstoneError(
                        HearthstoneError::LinkedWithoutManse,
                    ));
                }
                HearthstoneStability::ManseBorn => HearthstoneOrigin::ManseBorn(None),
                HearthstoneStability::ManseBornSteady => HearthstoneOrigin::ManseBornSteady(None),
                HearthstoneStability::Steady => HearthstoneOrigin::Steady(None),
                HearthstoneStability::WildBorn => {
                    return Err(CharacterMutationError::HearthstoneError(
                        HearthstoneError::WildBornWithManse,
                    ));
                }
                HearthstoneStability::Unspecified => HearthstoneOrigin::Unspecified(None),
            },
        };
        if let Entry::Vacant(e) = self.hearthstone_inventory.entry(hearthstone_name) {
            e.insert(unslotted);
            Ok(self)
        } else {
            Err(CharacterMutationError::HearthstoneError(
                HearthstoneError::DuplicateHearthstone,
            ))
        }
    }

    /// Slots a hearthstone into an artifact
    pub fn slot_hearthstone(
        &mut self,
        artifact_name: ArtifactName<'_>,
        hearthstone_name: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        let hearthstone = self.hearthstones().get(hearthstone_name).ok_or(
            CharacterMutationError::HearthstoneError(HearthstoneError::NotFound),
        )?;

        let maybe_slotted_into_name = hearthstone.slotted_into();
        if maybe_slotted_into_name == Some(artifact_name) {
            return Ok(self);
        }

        let (hearthstone_name, unslotted) = if let Some(slotted_into_id) = maybe_slotted_into_name {
            match slotted_into_id {
                ArtifactName::Weapon(artifact_weapon_name) => self
                    .exaltation
                    .unslot_hearthstone_from_weapon(artifact_weapon_name, hearthstone_name)?,
                ArtifactName::Armor(artifact_armor_name) => self
                    .exaltation
                    .unslot_hearthstone_from_armor(artifact_armor_name, hearthstone_name)?,
                ArtifactName::Wonder(wonder_name) => self
                    .exaltation
                    .unslot_hearthstone_from_wonder(wonder_name, hearthstone_name)?,
            }
        } else {
            self.hearthstone_inventory
                .remove_entry(hearthstone_name)
                .ok_or(CharacterMutationError::HearthstoneError(
                    HearthstoneError::NotFound,
                ))?
        };

        match artifact_name {
            ArtifactName::Weapon(artifact_weapon_name) => {
                if let Err(e) = self.exaltation.slot_hearthstone_into_weapon(
                    artifact_weapon_name,
                    hearthstone_name,
                    unslotted,
                ) {
                    // Something went wrong, put it back where it came from
                    if let Some(old_slotted_id) = maybe_slotted_into_name {
                        match old_slotted_id {
                            ArtifactName::Weapon(artifact_weapon_name) => {
                                self.exaltation.slot_hearthstone_into_weapon(
                                    artifact_weapon_name,
                                    hearthstone_name,
                                    unslotted,
                                )?
                            }
                            ArtifactName::Armor(artifact_armor_name) => {
                                self.exaltation.slot_hearthstone_into_armor(
                                    artifact_armor_name,
                                    hearthstone_name,
                                    unslotted,
                                )?
                            }
                            ArtifactName::Wonder(wonder_name) => {
                                self.exaltation.slot_hearthstone_into_wonder(
                                    wonder_name,
                                    hearthstone_name,
                                    unslotted,
                                )?
                            }
                        };
                    } else {
                        self.hearthstone_inventory
                            .insert(hearthstone_name, unslotted);
                    }
                    Err(e)
                } else {
                    Ok(self)
                }
            }
            ArtifactName::Armor(artifact_armor_id) => {
                if let Err(e) = self.exaltation.slot_hearthstone_into_armor(
                    artifact_armor_id,
                    hearthstone_name,
                    unslotted,
                ) {
                    // Something went wrong, put it back where it came from
                    if let Some(old_slotted_id) = maybe_slotted_into_name {
                        match old_slotted_id {
                            ArtifactName::Weapon(artifact_weapon_name) => {
                                self.exaltation.slot_hearthstone_into_weapon(
                                    artifact_weapon_name,
                                    hearthstone_name,
                                    unslotted,
                                )?
                            }
                            ArtifactName::Armor(artifact_armor_id) => {
                                self.exaltation.slot_hearthstone_into_armor(
                                    artifact_armor_id,
                                    hearthstone_name,
                                    unslotted,
                                )?
                            }
                            ArtifactName::Wonder(wonder_id) => {
                                self.exaltation.slot_hearthstone_into_wonder(
                                    wonder_id,
                                    hearthstone_name,
                                    unslotted,
                                )?
                            }
                        };
                    } else {
                        self.hearthstone_inventory
                            .insert(hearthstone_name, unslotted);
                    }
                    Err(e)
                } else {
                    Ok(self)
                }
            }
            ArtifactName::Wonder(wonder_id) => {
                if let Err(e) = self.exaltation.slot_hearthstone_into_wonder(
                    wonder_id,
                    hearthstone_name,
                    unslotted,
                ) {
                    // Something went wrong, put it back where it came from
                    if let Some(old_slotted_id) = maybe_slotted_into_name {
                        match old_slotted_id {
                            ArtifactName::Weapon(artifact_weapon_name) => {
                                self.exaltation.slot_hearthstone_into_weapon(
                                    artifact_weapon_name,
                                    hearthstone_name,
                                    unslotted,
                                )?
                            }
                            ArtifactName::Armor(artifact_armor_name) => {
                                self.exaltation.slot_hearthstone_into_armor(
                                    artifact_armor_name,
                                    hearthstone_name,
                                    unslotted,
                                )?
                            }
                            ArtifactName::Wonder(wonder_name) => {
                                self.exaltation.slot_hearthstone_into_wonder(
                                    wonder_name,
                                    hearthstone_name,
                                    unslotted,
                                )?
                            }
                        };
                    } else {
                        self.hearthstone_inventory
                            .insert(hearthstone_name, unslotted);
                    }
                    Err(e)
                } else {
                    Ok(self)
                }
            }
        }
    }

    /// Unslots a hearthstone from wherever it currently is
    pub fn unslot_hearthstone(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        let (unslotted_name, unslotted) = match self
            .hearthstones()
            .get(name)
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::NotFound,
            ))?
            .slotted_into()
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::NotSlotted,
            ))? {
            ArtifactName::Weapon(artifact_weapon_name) => self
                .exaltation
                .unslot_hearthstone_from_weapon(artifact_weapon_name, name)?,
            ArtifactName::Armor(artifact_armor_name) => self
                .exaltation
                .unslot_hearthstone_from_armor(artifact_armor_name, name)?,
            ArtifactName::Wonder(wonder_name) => self
                .exaltation
                .unslot_hearthstone_from_wonder(wonder_name, name)?,
        };

        self.hearthstone_inventory.insert(unslotted_name, unslotted);
        Ok(self)
    }

    /// Removes a hearthstone from a character.
    pub fn remove_hearthstone(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        if self
            .hearthstones()
            .get(name)
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::NotFound,
            ))?
            .slotted_into()
            .is_some()
        {
            self.unslot_hearthstone(name)?;
        }

        self.hearthstone_inventory
            .remove(name)
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::NotFound,
            ))?;
        // May lose evocations along with the hearthstone
        self.correct_evocations(&[]);
        Ok(self)
    }
}
