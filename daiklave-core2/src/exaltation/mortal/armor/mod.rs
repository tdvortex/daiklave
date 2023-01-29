mod memo;
pub(crate) use memo::MortalArmorMemo;

use std::collections::{hash_map::Entry, HashMap};

use crate::{
    armor::{
        armor_item::{
            artifact::{ArtifactArmorNoAttunement, ArtifactArmorView, ArtifactError},
            mundane::{MundaneArmor, MundaneArmorView},
            ArmorItem, ArmorName, ArmorType, EquippedArmor, EquippedArmorNoAttunement,
        },
        ArmorError,
    },
    exaltation::exalt::ExaltArmor,
    hearthstones::{HearthstoneError, SlottedHearthstone, UnslottedHearthstone},
    CharacterMutationError,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MortalArmor<'source> {
    pub equipped: Option<EquippedArmorNoAttunement<'source>>,
    pub unequipped_mundane: HashMap<&'source str, MundaneArmorView<'source>>,
    pub unequipped_artifact: HashMap<&'source str, ArtifactArmorNoAttunement<'source>>,
}

impl<'source> MortalArmor<'source> {
    pub fn worn_armor(&self) -> Option<ArmorItem<'source>> {
        if let Some(equipped) = &self.equipped {
            match equipped {
                EquippedArmorNoAttunement::Mundane(id, mundane) => {
                    Some(ArmorItem(ArmorType::Mundane(*id, *mundane), true))
                }
                EquippedArmorNoAttunement::Artifact(id, artifact) => Some(ArmorItem(
                    ArmorType::Artifact(*id, artifact.clone(), None),
                    true,
                )),
            }
        } else {
            None
        }
    }

    pub fn iter(&self) -> std::vec::IntoIter<ArmorName<'source>> {
        self.worn_armor()
            .iter()
            .map(|item| item.name())
            .chain(
                self.unequipped_mundane
                    .keys()
                    .map(|k| ArmorName::Mundane(*k)),
            )
            .chain(
                self.unequipped_artifact
                    .keys()
                    .map(|k| ArmorName::Artifact(*k)),
            )
            .collect::<Vec<ArmorName>>()
            .into_iter()
    }

    pub fn get(&self, name: ArmorName<'_>) -> Option<ArmorItem<'source>> {
        let unequipped = match name {
            ArmorName::Mundane(name) => {
                self.unequipped_mundane
                    .get_key_value(name)
                    .map(|(name, mundane_armor)| {
                        ArmorItem(ArmorType::Mundane(*name, *mundane_armor), false)
                    })
            }
            ArmorName::Artifact(name) => {
                self.unequipped_artifact
                    .get_key_value(name)
                    .map(|(name, no_attunement)| {
                        ArmorItem(
                            ArmorType::Artifact(*name, no_attunement.clone(), None),
                            false,
                        )
                    })
            }
        };

        if unequipped.is_some() {
            unequipped
        } else if let Some(equipped) = self.worn_armor() {
            if equipped.name() == name {
                Some(equipped)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn add_mundane(
        &mut self,
        name: &'source str,
        armor: &'source MundaneArmor,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self
            .worn_armor()
            .map_or(false, |item| item.name() == ArmorName::Mundane(name))
        {
            Err(CharacterMutationError::ArmorError(
                ArmorError::DuplicateArmor,
            ))
        } else if let Entry::Vacant(e) = self.unequipped_mundane.entry(name) {
            e.insert(armor.into());
            Ok(self)
        } else {
            Err(CharacterMutationError::ArmorError(
                ArmorError::DuplicateArmor,
            ))
        }
    }

    pub fn remove_mundane(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        if self.unequipped_mundane.remove(name).is_some() {
            Ok(self)
        } else if self
            .worn_armor()
            .map_or(false, |item| item.name() == ArmorName::Mundane(name))
        {
            Err(CharacterMutationError::ArmorError(
                ArmorError::RemoveEquipped,
            ))
        } else {
            Err(CharacterMutationError::ArmorError(ArmorError::NotFound))
        }
    }

    pub fn unequip(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if let Some(equipped) = self.equipped.take() {
            match equipped {
                EquippedArmorNoAttunement::Mundane(base_armor_id, mundane_armor) => {
                    self.unequipped_mundane.insert(base_armor_id, mundane_armor);
                    Ok(self)
                }
                EquippedArmorNoAttunement::Artifact(artifact_armor_id, artifact_armor) => {
                    self.unequipped_artifact
                        .insert(artifact_armor_id, artifact_armor);
                    Ok(self)
                }
            }
        } else {
            Err(CharacterMutationError::ArmorError(ArmorError::NotFound))
        }
    }

    pub fn equip(&mut self, name: ArmorName<'_>) -> Result<&mut Self, CharacterMutationError> {
        let unstowed = match name {
            ArmorName::Mundane(name) => {
                let (name, mundane) = self
                    .unequipped_mundane
                    .remove_entry(name)
                    .ok_or(CharacterMutationError::ArmorError(ArmorError::NotFound))?;
                EquippedArmorNoAttunement::Mundane(name, mundane)
            }
            ArmorName::Artifact(name) => {
                let (name, artifact) = self
                    .unequipped_artifact
                    .remove_entry(name)
                    .ok_or(CharacterMutationError::ArmorError(ArmorError::NotFound))?;
                EquippedArmorNoAttunement::Artifact(name, artifact)
            }
        };

        if let Some(old_equipped) = self.equipped.take() {
            match old_equipped {
                EquippedArmorNoAttunement::Mundane(base_armor_id, mundane_armor) => {
                    self.unequipped_mundane.insert(base_armor_id, mundane_armor);
                }
                EquippedArmorNoAttunement::Artifact(artifact_armor_id, artifact_armor) => {
                    self.unequipped_artifact
                        .insert(artifact_armor_id, artifact_armor);
                }
            }
        }

        self.equipped = Some(unstowed);
        Ok(self)
    }

    pub fn add_artifact(
        &mut self,
        name: &'source str,
        armor: ArtifactArmorView<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self
            .worn_armor()
            .map_or(false, |item| item.name() == ArmorName::Artifact(name))
        {
            Err(CharacterMutationError::ArtifactError(
                ArtifactError::NamedArtifactsUnique,
            ))
        } else if let Entry::Vacant(e) = self.unequipped_artifact.entry(name) {
            e.insert(armor.0);
            Ok(self)
        } else {
            Err(CharacterMutationError::ArmorError(
                ArmorError::DuplicateArmor,
            ))
        }
    }

    pub fn remove_artifact(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        if self.unequipped_artifact.remove(name).is_some() {
            Ok(self)
        } else if self
            .worn_armor()
            .map_or(false, |item| item.name() == ArmorName::Artifact(name))
        {
            Err(CharacterMutationError::ArmorError(
                ArmorError::RemoveEquipped,
            ))
        } else {
            Err(CharacterMutationError::ArmorError(ArmorError::NotFound))
        }
    }

    pub fn slot_hearthstone(
        &mut self,
        artifact_armor_name: &str,
        hearthstone_name: &'source str,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        *self
            .equipped
            .as_mut()
            .and_then(|equipped| match equipped {
                EquippedArmorNoAttunement::Mundane(_, _) => None,
                EquippedArmorNoAttunement::Artifact(worn_id, worn_armor) => {
                    if worn_id == &artifact_armor_name {
                        Some(worn_armor)
                    } else {
                        None
                    }
                }
            })
            .or_else(|| self.unequipped_artifact.get_mut(artifact_armor_name))
            .ok_or(CharacterMutationError::ArmorError(ArmorError::NotFound))?
            .hearthstone_slots
            .iter_mut()
            .find(|maybe_hearthstone| maybe_hearthstone.is_none())
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::AllSlotsFilled,
            ))? = Some(SlottedHearthstone {
            name: hearthstone_name,
            details: unslotted.details,
            origin: unslotted.origin,
        });
        Ok(self)
    }

    pub fn unslot_hearthstone(
        &mut self,
        artifact_armor_name: &str,
        hearthstone_name: &str,
    ) -> Result<(&'source str, UnslottedHearthstone<'source>), CharacterMutationError> {
        let SlottedHearthstone {
            name,
            details,
            origin,
        } = self
            .equipped
            .as_mut()
            .and_then(|equipped| match equipped {
                EquippedArmorNoAttunement::Mundane(_, _) => None,
                EquippedArmorNoAttunement::Artifact(worn_id, worn_armor) => {
                    if worn_id == &artifact_armor_name {
                        Some(worn_armor)
                    } else {
                        None
                    }
                }
            })
            .or_else(|| self.unequipped_artifact.get_mut(artifact_armor_name))
            .ok_or(CharacterMutationError::ArmorError(ArmorError::NotFound))?
            .hearthstone_slots
            .iter_mut()
            .find_map(|maybe_hearthstone| {
                if maybe_hearthstone
                    .map_or(false, |hearthstone| hearthstone.name == hearthstone_name)
                {
                    maybe_hearthstone.take()
                } else {
                    None
                }
            })
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::NotFound,
            ))?;

        Ok((name, UnslottedHearthstone { details, origin }))
    }
}

impl<'source> From<ExaltArmor<'source>> for MortalArmor<'source> {
    fn from(exalt: ExaltArmor<'source>) -> Self {
        Self {
            equipped: exalt.equipped.map(|maybe_attuned| match maybe_attuned {
                EquippedArmor::Mundane(base_armor_id, mundane_armor) => {
                    EquippedArmorNoAttunement::Mundane(base_armor_id, mundane_armor)
                }
                EquippedArmor::Artifact(artifact_armor_id, artifact_armor) => {
                    EquippedArmorNoAttunement::Artifact(artifact_armor_id, artifact_armor.0)
                }
            }),
            unequipped_mundane: exalt.unequipped_mundane,
            unequipped_artifact: exalt
                .unequipped_artifact
                .into_iter()
                .map(|(k, v)| (k, v.0))
                .collect(),
        }
    }
}
