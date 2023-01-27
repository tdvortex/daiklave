mod memo;
pub(crate) use memo::ExaltArmorMemo;

use std::collections::{hash_map::Entry, HashMap};

use crate::{
    armor::{
        armor_item::{
            artifact::{ArtifactArmorView, ArtifactError},
            mundane::{MundaneArmor, MundaneArmorView},
            ArmorItem, ArmorName, ArmorType, ArmorWeightClass, EquippedArmor,
            EquippedArmorNoAttunement,
        },
        ArmorError,
    },
    exaltation::mortal::MortalArmor,
    hearthstones::{HearthstoneError, HearthstoneId, SlottedHearthstone, UnslottedHearthstone},
    CharacterMutationError,
};

use super::essence::EssenceError;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ExaltArmor<'source> {
    pub equipped: Option<EquippedArmor<'source>>,
    pub unequipped_mundane: HashMap<&'source str, MundaneArmorView<'source>>,
    pub unequipped_artifact: HashMap<&'source str, ArtifactArmorView<'source>>,
}

impl<'source> ExaltArmor<'source> {
    pub fn as_memo(&self) -> ExaltArmorMemo {
        ExaltArmorMemo {
            equipped: self.equipped.as_ref().map(|view| view.as_memo()),
            unequipped_mundane: self
                .unequipped_mundane
                .iter()
                .map(|(k, v)| ((*k).to_owned(), v.as_memo()))
                .collect(),
            unequipped_artifact: self
                .unequipped_artifact
                .iter()
                .map(|(k, v)| ((*k).to_owned(), v.as_memo()))
                .collect(),
        }
    }

    pub fn worn_armor(&self) -> Option<ArmorItem<'source>> {
        if let Some(equipped) = &self.equipped {
            match equipped {
                EquippedArmor::Mundane(name, mundane) => {
                    Some(ArmorItem(ArmorType::Mundane(*name, *mundane), true))
                }
                EquippedArmor::Artifact(name, artifact) => {
                    let (no_attunement, attunement) = (&artifact.0, artifact.1);
                    Some(ArmorItem(
                        ArmorType::Artifact(*name, no_attunement.clone(), attunement),
                        true,
                    ))
                }
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
                    .map(|(name, artifact_armor)| {
                        let (no_attunement, attunement) = (&artifact_armor.0, artifact_armor.1);
                        ArmorItem(
                            ArmorType::Artifact(*name, no_attunement.clone(), attunement),
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
            e.insert(armor.as_ref());
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
                EquippedArmor::Mundane(name, mundane_armor) => {
                    self.unequipped_mundane.insert(name, mundane_armor);
                    Ok(self)
                }
                EquippedArmor::Artifact(name, artifact_armor) => {
                    self.unequipped_artifact.insert(name, artifact_armor);
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
                let (name, mundane_armor) = self
                    .unequipped_mundane
                    .remove_entry(name)
                    .ok_or(CharacterMutationError::ArmorError(ArmorError::NotFound))?;
                EquippedArmor::Mundane(name, mundane_armor)
            }
            ArmorName::Artifact(name) => {
                let (name, artifact_armor) = self
                    .unequipped_artifact
                    .remove_entry(name)
                    .ok_or(CharacterMutationError::ArmorError(ArmorError::NotFound))?;

                EquippedArmor::Artifact(name, artifact_armor)
            }
        };

        if let Some(old_equipped) = self.equipped.take() {
            match old_equipped {
                EquippedArmor::Mundane(base_armor_id, mundane_armor) => {
                    self.unequipped_mundane.insert(base_armor_id, mundane_armor);
                }
                EquippedArmor::Artifact(artifact_armor_id, artifact_armor) => {
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
            // Artifacts are always added unattuned
            let no_attunement = armor.0;
            e.insert(ArtifactArmorView(no_attunement, None));
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
        hearthstone_id: HearthstoneId,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        *self
            .equipped
            .as_mut()
            .and_then(|equipped| match equipped {
                EquippedArmor::Mundane(_, _) => None,
                EquippedArmor::Artifact(worn_name, worn_armor) => {
                    if worn_name == &artifact_armor_name {
                        Some(worn_armor)
                    } else {
                        None
                    }
                }
            })
            .or_else(|| self.unequipped_artifact.get_mut(artifact_armor_name))
            .ok_or(CharacterMutationError::ArmorError(ArmorError::NotFound))?
            .0
            .hearthstone_slots
            .iter_mut()
            .find(|maybe_hearthstone| maybe_hearthstone.is_none())
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::AllSlotsFilled,
            ))? = Some(SlottedHearthstone {
            hearthstone_id,
            details: unslotted.details,
            origin: unslotted.origin,
        });
        Ok(self)
    }

    pub fn unslot_hearthstone(
        &mut self,
        artifact_armor_name: &str,
        hearthstone_id: HearthstoneId,
    ) -> Result<UnslottedHearthstone<'source>, CharacterMutationError> {
        let SlottedHearthstone {
            hearthstone_id: _,
            details,
            origin,
        } = self
            .equipped
            .as_mut()
            .and_then(|equipped| match equipped {
                EquippedArmor::Mundane(_, _) => None,
                EquippedArmor::Artifact(worn_id, worn_armor) => {
                    if worn_id == &artifact_armor_name {
                        Some(worn_armor)
                    } else {
                        None
                    }
                }
            })
            .or_else(|| self.unequipped_artifact.get_mut(artifact_armor_name))
            .ok_or(CharacterMutationError::ArmorError(ArmorError::NotFound))?
            .0
            .hearthstone_slots
            .iter_mut()
            .find_map(|maybe_hearthstone| {
                if maybe_hearthstone.map_or(false, |hearthstone| hearthstone.id() == hearthstone_id)
                {
                    maybe_hearthstone.take()
                } else {
                    None
                }
            })
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::NotFound,
            ))?;

        Ok(UnslottedHearthstone { details, origin })
    }

    pub fn attune_artifact_armor(
        &mut self,
        artifact_armor_name: &str,
        personal_committed: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        let attunement = &mut self
            .equipped
            .as_mut()
            .and_then(|equipped| match equipped {
                EquippedArmor::Mundane(_, _) => None,
                EquippedArmor::Artifact(worn_id, worn_armor) => {
                    if worn_id == &artifact_armor_name {
                        Some(worn_armor)
                    } else {
                        None
                    }
                }
            })
            .or_else(|| self.unequipped_artifact.get_mut(artifact_armor_name))
            .ok_or(CharacterMutationError::ArmorError(ArmorError::NotFound))?
            .1;

        if attunement.is_none() {
            *attunement = Some(personal_committed);
            Ok(self)
        } else {
            Err(CharacterMutationError::EssenceError(
                EssenceError::AlreadyAttuned,
            ))
        }
    }

    pub fn unattune_artifact_armor(
        &mut self,
        artifact_armor_name: &str,
    ) -> Result<(u8, u8), CharacterMutationError> {
        let armor = self
            .equipped
            .as_mut()
            .and_then(|equipped| match equipped {
                EquippedArmor::Mundane(_, _) => None,
                EquippedArmor::Artifact(worn_id, worn_armor) => {
                    if worn_id == &artifact_armor_name {
                        Some(worn_armor)
                    } else {
                        None
                    }
                }
            })
            .or_else(|| self.unequipped_artifact.get_mut(artifact_armor_name))
            .ok_or(CharacterMutationError::ArmorError(ArmorError::NotFound))?;

        let amount = match armor.0.base_armor().weight_class() {
            ArmorWeightClass::Light => 4,
            ArmorWeightClass::Medium => 5,
            ArmorWeightClass::Heavy => 6,
        };

        if let Some(personal) = armor.1.take() {
            Ok((amount - amount.min(personal), amount.min(personal)))
        } else {
            Err(CharacterMutationError::EssenceError(EssenceError::NotFound))
        }
    }
}

impl<'source> From<MortalArmor<'source>> for ExaltArmor<'source> {
    fn from(mortal: MortalArmor<'source>) -> Self {
        Self {
            equipped: mortal.equipped.map(|no_attunement| match no_attunement {
                EquippedArmorNoAttunement::Mundane(base_armor_id, mundane_armor) => {
                    EquippedArmor::Mundane(base_armor_id, mundane_armor)
                }
                EquippedArmorNoAttunement::Artifact(artifact_armor_id, no_attunement) => {
                    EquippedArmor::Artifact(
                        artifact_armor_id,
                        ArtifactArmorView(no_attunement, None),
                    )
                }
            }),
            unequipped_mundane: mortal.unequipped_mundane,
            unequipped_artifact: mortal
                .unequipped_artifact
                .into_iter()
                .map(|(k, v)| (k, ArtifactArmorView(v, None)))
                .collect(),
        }
    }
}
