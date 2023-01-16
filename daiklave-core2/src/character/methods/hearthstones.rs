use crate::{
    armor::{
        armor_item::{artifact::ArtifactError, ArmorId},
        ArmorError,
    },
    artifact::ArtifactId,
    hearthstones::{
        hearthstone::HearthstoneTemplate, HearthstoneError, HearthstoneId, HearthstoneOrigin,
        HearthstoneStability, Hearthstones, UnslottedHearthstone,
    },
    weapons::{weapon::WeaponId, WeaponError},
    Character, CharacterMutationError,
};

impl<'view, 'source> Character<'source> {
    /// Gets the character's Hearthstones.
    pub fn hearthstones(&'view self) -> Hearthstones<'view, 'source> {
        Hearthstones(self)
    }

    /// Checks if a manse (with demense and hearthstone) can be added
    pub fn check_add_manse(
        &self,
        _manse_name: &str,
        _demense_name: &str,
        hearthstone_id: HearthstoneId,
        template: &HearthstoneTemplate,
    ) -> Result<(), CharacterMutationError> {
        if self.hearthstones().get(hearthstone_id).is_some() {
            Err(CharacterMutationError::HearthstoneError(
                HearthstoneError::UniqueHearthstone,
            ))
        } else if let HearthstoneStability::WildBorn = template.stability {
            Err(CharacterMutationError::HearthstoneError(
                HearthstoneError::WildBornWithManse,
            ))
        } else {
            Ok(())
        }
    }

    /// Adds a manse (and demense and hearthstone) to the character.
    pub fn add_manse(
        &mut self,
        manse_name: &'source str,
        demense_name: &'source str,
        hearthstone_id: HearthstoneId,
        template: &'source HearthstoneTemplate,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_add_manse(manse_name, demense_name, hearthstone_id, template)?;

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
        self.hearthstone_inventory.insert(hearthstone_id, unslotted);
        Ok(self)
    }

    /// Checks if a standalone hearthstone (without a manse) can be added to
    /// the character.
    pub fn check_add_hearthstone(
        &self,
        hearthstone_id: HearthstoneId,
        template: &HearthstoneTemplate,
    ) -> Result<(), CharacterMutationError> {
        if self.hearthstones().get(hearthstone_id).is_some() {
            Err(CharacterMutationError::HearthstoneError(
                HearthstoneError::UniqueHearthstone,
            ))
        } else if let HearthstoneStability::Linked = template.stability {
            Err(CharacterMutationError::HearthstoneError(
                HearthstoneError::LinkedWithoutManse,
            ))
        } else {
            Ok(())
        }
    }

    /// Adds a standalone hearthstone (without a manse) to the character.
    pub fn add_hearthstone(
        &mut self,
        hearthstone_id: HearthstoneId,
        template: &'source HearthstoneTemplate,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_add_hearthstone(hearthstone_id, template)?;

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
        self.hearthstone_inventory.insert(hearthstone_id, unslotted);
        Ok(self)
    }

    /// Checks if a hearthstone can be slotted into an artifact.
    pub fn check_slot_hearthstone(
        &self,
        artifact_id: ArtifactId,
        hearthstone_id: HearthstoneId,
    ) -> Result<(), CharacterMutationError> {
        if self.hearthstones().get(hearthstone_id).is_none() {
            Err(CharacterMutationError::HearthstoneError(
                HearthstoneError::NotFound,
            ))
        } else {
            match artifact_id {
                ArtifactId::Weapon(artifact_weapon_id) => {
                    if self
                        .weapons()
                        .iter()
                        .find(|(weapon_id, _equipped)| {
                            weapon_id == &WeaponId::Artifact(artifact_weapon_id)
                        })
                        .and_then(|(weapon_id, equipped)| self.weapons().get(weapon_id, equipped))
                        .ok_or(CharacterMutationError::WeaponError(WeaponError::NotFound))?
                        .open_slots()
                        < 1
                    {
                        Err(CharacterMutationError::HearthstoneError(
                            HearthstoneError::AllSlotsFilled,
                        ))
                    } else {
                        Ok(())
                    }
                }
                ArtifactId::Armor(artifact_armor_id) => {
                    if self
                        .armor()
                        .get(ArmorId::Artifact(artifact_armor_id))
                        .ok_or(CharacterMutationError::ArmorError(ArmorError::NotFound))?
                        .open_slots()
                        < 1
                    {
                        Err(CharacterMutationError::HearthstoneError(
                            HearthstoneError::AllSlotsFilled,
                        ))
                    } else {
                        Ok(())
                    }
                }
                ArtifactId::Wonder(wonder_id) => {
                    if self
                        .wonders()
                        .get(wonder_id)
                        .ok_or(CharacterMutationError::ArtifactError(
                            ArtifactError::NotFound,
                        ))?
                        .open_slots()
                        < 1
                    {
                        Err(CharacterMutationError::HearthstoneError(
                            HearthstoneError::AllSlotsFilled,
                        ))
                    } else {
                        Ok(())
                    }
                }
            }
        }
    }

    /// Slots a hearthstone into an artifact
    pub fn slot_hearthstone(
        &mut self,
        artifact_id: ArtifactId,
        hearthstone_id: HearthstoneId,
    ) -> Result<&mut Self, CharacterMutationError> {
        let maybe_slotted_into_id = self
            .hearthstones()
            .get(hearthstone_id)
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::NotFound,
            ))?
            .slotted_into();
        if maybe_slotted_into_id == Some(artifact_id) {
            return Ok(self);
        }

        let unslotted = if let Some(slotted_into_id) = maybe_slotted_into_id {
            match slotted_into_id {
                ArtifactId::Weapon(artifact_weapon_id) => self
                    .exaltation
                    .unslot_hearthstone_from_weapon(artifact_weapon_id, hearthstone_id)?,
                ArtifactId::Armor(artifact_armor_id) => self
                    .exaltation
                    .unslot_hearthstone_from_armor(artifact_armor_id, hearthstone_id)?,
                ArtifactId::Wonder(wonder_id) => self
                    .exaltation
                    .unslot_hearthstone_from_wonder(wonder_id, hearthstone_id)?,
            }
        } else {
            self.hearthstone_inventory.remove(&hearthstone_id).ok_or(
                CharacterMutationError::HearthstoneError(HearthstoneError::NotFound),
            )?
        };

        match artifact_id {
            ArtifactId::Weapon(artifact_weapon_id) => {
                if let Err(e) = self.exaltation.slot_hearthstone_into_weapon(
                    artifact_weapon_id,
                    hearthstone_id,
                    unslotted,
                ) {
                    // Something went wrong, put it back where it came from
                    if let Some(old_slotted_id) = maybe_slotted_into_id {
                        match old_slotted_id {
                            ArtifactId::Weapon(artifact_weapon_id) => {
                                self.exaltation.slot_hearthstone_into_weapon(
                                    artifact_weapon_id,
                                    hearthstone_id,
                                    unslotted,
                                )?
                            }
                            ArtifactId::Armor(artifact_armor_id) => {
                                self.exaltation.slot_hearthstone_into_armor(
                                    artifact_armor_id,
                                    hearthstone_id,
                                    unslotted,
                                )?
                            }
                            ArtifactId::Wonder(wonder_id) => {
                                self.exaltation.slot_hearthstone_into_wonder(
                                    wonder_id,
                                    hearthstone_id,
                                    unslotted,
                                )?
                            }
                        };
                    } else {
                        self.hearthstone_inventory.insert(hearthstone_id, unslotted);
                    }
                    Err(e)
                } else {
                    Ok(self)
                }
            }
            ArtifactId::Armor(artifact_armor_id) => {
                if let Err(e) = self.exaltation.slot_hearthstone_into_armor(
                    artifact_armor_id,
                    hearthstone_id,
                    unslotted,
                ) {
                    // Something went wrong, put it back where it came from
                    if let Some(old_slotted_id) = maybe_slotted_into_id {
                        match old_slotted_id {
                            ArtifactId::Weapon(artifact_weapon_id) => {
                                self.exaltation.slot_hearthstone_into_weapon(
                                    artifact_weapon_id,
                                    hearthstone_id,
                                    unslotted,
                                )?
                            }
                            ArtifactId::Armor(artifact_armor_id) => {
                                self.exaltation.slot_hearthstone_into_armor(
                                    artifact_armor_id,
                                    hearthstone_id,
                                    unslotted,
                                )?
                            }
                            ArtifactId::Wonder(wonder_id) => {
                                self.exaltation.slot_hearthstone_into_wonder(
                                    wonder_id,
                                    hearthstone_id,
                                    unslotted,
                                )?
                            }
                        };
                    } else {
                        self.hearthstone_inventory.insert(hearthstone_id, unslotted);
                    }
                    Err(e)
                } else {
                    Ok(self)
                }
            }
            ArtifactId::Wonder(wonder_id) => {
                if let Err(e) = self.exaltation.slot_hearthstone_into_wonder(
                    wonder_id,
                    hearthstone_id,
                    unslotted,
                ) {
                    // Something went wrong, put it back where it came from
                    if let Some(old_slotted_id) = maybe_slotted_into_id {
                        match old_slotted_id {
                            ArtifactId::Weapon(artifact_weapon_id) => {
                                self.exaltation.slot_hearthstone_into_weapon(
                                    artifact_weapon_id,
                                    hearthstone_id,
                                    unslotted,
                                )?
                            }
                            ArtifactId::Armor(artifact_armor_id) => {
                                self.exaltation.slot_hearthstone_into_armor(
                                    artifact_armor_id,
                                    hearthstone_id,
                                    unslotted,
                                )?
                            }
                            ArtifactId::Wonder(wonder_id) => {
                                self.exaltation.slot_hearthstone_into_wonder(
                                    wonder_id,
                                    hearthstone_id,
                                    unslotted,
                                )?
                            }
                        };
                    } else {
                        self.hearthstone_inventory.insert(hearthstone_id, unslotted);
                    }
                    Err(e)
                } else {
                    Ok(self)
                }
            }
        }
    }

    /// Checks if a hearthstone can be unslotted from wherever it currently is
    pub fn check_unslot_hearthstone(
        &self,
        hearthstone_id: HearthstoneId,
    ) -> Result<(), CharacterMutationError> {
        self.hearthstones()
            .get(hearthstone_id)
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::NotFound,
            ))?
            .slotted_into()
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::NotSlotted,
            ))?;
        Ok(())
    }

    /// Unslots a hearthstone from wherever it currently is
    pub fn unslot_hearthstone(
        &mut self,
        hearthstone_id: HearthstoneId,
    ) -> Result<&mut Self, CharacterMutationError> {
        let unslotted = match self
            .hearthstones()
            .get(hearthstone_id)
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::NotFound,
            ))?
            .slotted_into()
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::NotSlotted,
            ))? {
            ArtifactId::Weapon(artifact_weapon_id) => self
                .exaltation
                .unslot_hearthstone_from_weapon(artifact_weapon_id, hearthstone_id)?,
            ArtifactId::Armor(artifact_armor_id) => self
                .exaltation
                .unslot_hearthstone_from_armor(artifact_armor_id, hearthstone_id)?,
            ArtifactId::Wonder(wonder_id) => self
                .exaltation
                .unslot_hearthstone_from_wonder(wonder_id, hearthstone_id)?,
        };

        self.hearthstone_inventory.insert(hearthstone_id, unslotted);
        Ok(self)
    }

    /// Checks if a hearthstone can be removed.
    pub fn check_remove_hearthstone(
        &self,
        hearthstone_id: HearthstoneId,
    ) -> Result<(), CharacterMutationError> {
        if self.hearthstones().get(hearthstone_id).is_none() {
            Err(CharacterMutationError::HearthstoneError(
                HearthstoneError::NotFound,
            ))
        } else {
            Ok(())
        }
    }

    /// Removes a hearthstone from a character.
    pub fn remove_hearthstone(
        &mut self,
        hearthstone_id: HearthstoneId,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self
            .hearthstones()
            .get(hearthstone_id)
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::NotFound,
            ))?
            .slotted_into()
            .is_some()
        {
            self.unslot_hearthstone(hearthstone_id)?;
        }

        self.hearthstone_inventory.remove(&hearthstone_id).ok_or(
            CharacterMutationError::HearthstoneError(HearthstoneError::NotFound),
        )?;
        Ok(self)
    }
}
