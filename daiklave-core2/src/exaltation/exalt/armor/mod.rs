use std::collections::{HashMap, hash_map::Entry};

use crate::{armor::{armor_item::{
    artifact::{ArtifactArmor, ArtifactArmorId},
    mundane::{MundaneArmor, MundaneArmorMemo},
    ArmorId, ArmorItem, ArmorType, BaseArmorId, EquippedArmor,
}, ArmorError}, CharacterMutationError};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ExaltArmor<'source> {
    equipped: Option<EquippedArmor<'source>>,
    unequipped_mundane: HashMap<BaseArmorId, MundaneArmor<'source>>,
    unequipped_artifact: HashMap<ArtifactArmorId, ArtifactArmor<'source>>,
}

impl<'source> ExaltArmor<'source> {
    pub fn worn_armor(&self) -> Option<ArmorItem<'source>> {
        if let Some(equipped) = &self.equipped {
            match equipped {
                EquippedArmor::Mundane(id, mundane) => {
                    Some(ArmorItem(ArmorType::Mundane(*id, *mundane), true))
                }
                EquippedArmor::Artifact(id, artifact) => {
                    let (no_attunement, attunement) = (&artifact.0, artifact.1);
                    Some(ArmorItem(
                        ArmorType::Artifact(*id, no_attunement.clone(), attunement),
                        true,
                    ))
                }
            }
        } else {
            None
        }
    }

    pub fn iter(&self) -> std::vec::IntoIter<ArmorId> {
        self.worn_armor()
            .iter()
            .map(|item| item.id())
            .chain(self.unequipped_mundane.keys().map(|k| ArmorId::Mundane(*k)))
            .chain(
                self.unequipped_artifact
                    .keys()
                    .map(|k| ArmorId::Artifact(*k)),
            ).collect::<Vec<ArmorId>>().into_iter()
    }

    pub fn get(&self, armor_id: ArmorId) -> Option<ArmorItem<'source>> {
        let unequipped = match armor_id {
            ArmorId::Mundane(base_armor_id) => {
                if let Some(mundane_armor) = self.unequipped_mundane.get(&base_armor_id) {
                    Some(ArmorItem(
                        ArmorType::Mundane(base_armor_id, *mundane_armor),
                        false
                    ))
                } else {
                    None
                }
            }
            ArmorId::Artifact(artifact_armor_id) => {
                if let Some(artifact_armor) = self.unequipped_artifact.get(&artifact_armor_id) {
                    let (no_attunement, attunement) = (&artifact_armor.0, artifact_armor.1);
                    Some(ArmorItem(ArmorType::Artifact(artifact_armor_id, no_attunement.clone(), attunement), false))
                } else {
                    None
                }
            }
        };

        if unequipped.is_some() {
            unequipped
        } else if let Some(equipped) = self.worn_armor() {
            if equipped.id() == armor_id {
                Some(equipped)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn add_mundane(&mut self, armor_id: BaseArmorId, armor: &'source MundaneArmorMemo) -> Result<&mut Self, CharacterMutationError> {
        if self.worn_armor().map_or(false, |item| item.id() == ArmorId::Mundane(armor_id)) {
            Err(CharacterMutationError::ArmorError(ArmorError::DuplicateArmor))
        } else if let Entry::Vacant(e) = self.unequipped_mundane.entry(armor_id) {
            e.insert(armor.as_ref());
            Ok(self)
        } else {
            Err(CharacterMutationError::ArmorError(ArmorError::DuplicateArmor))
        }
    }

    pub fn remove_mundane(&mut self, armor_id: BaseArmorId) -> Result<&mut Self, CharacterMutationError> {
        if self.unequipped_mundane.remove(&armor_id).is_some() {
            Ok(self)
        } else if self.worn_armor().map_or(false, |item| item.id() == ArmorId::Mundane(armor_id)) {
            Err(CharacterMutationError::ArmorError(ArmorError::RemoveEquipped))
        } else {
            Err(CharacterMutationError::ArmorError(ArmorError::NotFound))
        }
    }

    pub fn unequip(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if let Some(equipped) = self.equipped.take() {
            match equipped {
                EquippedArmor::Mundane(base_armor_id, mundane_armor) => {
                    self.unequipped_mundane.insert(base_armor_id, mundane_armor);
                    Ok(self)
                }
                EquippedArmor::Artifact(artifact_armor_id, artifact_armor) => {
                    self.unequipped_artifact.insert(artifact_armor_id, artifact_armor);
                    Ok(self)
                }
            }
        } else {
            Err(CharacterMutationError::ArmorError(ArmorError::NotFound))
        }
    }

    pub fn equip(&mut self, armor_id: ArmorId) -> Result<&mut Self, CharacterMutationError> {
        let unstowed = match armor_id {
            ArmorId::Mundane(base_armor_id) => EquippedArmor::Mundane(base_armor_id, self.unequipped_mundane.remove(&base_armor_id).ok_or(CharacterMutationError::ArmorError(ArmorError::NotFound))?),
            ArmorId::Artifact(artifact_armor_id) => EquippedArmor::Artifact(artifact_armor_id, self.unequipped_artifact.remove(&artifact_armor_id).ok_or(CharacterMutationError::ArmorError(ArmorError::NotFound))?),
        };

        if let Some(old_equipped) = self.equipped.take() {
            match old_equipped {
                EquippedArmor::Mundane(base_armor_id, mundane_armor) => {
                    self.unequipped_mundane.insert(base_armor_id, mundane_armor);
                }
                EquippedArmor::Artifact(artifact_armor_id, artifact_armor) => {
                    self.unequipped_artifact.insert(artifact_armor_id, artifact_armor);
                }
            }
        }

        self.equipped = Some(unstowed);
        Ok(self)
    }
}
