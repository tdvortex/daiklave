mod memo;
pub(crate) use memo::MortalArmorMemo;

use std::collections::{hash_map::Entry, HashMap};

use crate::{
    armor::{
        armor_item::{
            artifact::{ArtifactArmorView, ArtifactArmorId, ArtifactArmorNoAttunement, ArtifactError},
            mundane::{MundaneArmorView, MundaneArmor},
            ArmorId, ArmorItem, ArmorType, BaseArmorId, EquippedArmor, EquippedArmorNoAttunement,
        },
        ArmorError,
    },
    exaltation::exalt::ExaltArmor,
    CharacterMutationError,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MortalArmor<'source> {
    pub equipped: Option<EquippedArmorNoAttunement<'source>>,
    pub unequipped_mundane: HashMap<BaseArmorId, MundaneArmorView<'source>>,
    pub unequipped_artifact: HashMap<ArtifactArmorId, ArtifactArmorNoAttunement<'source>>,
}

impl<'source> MortalArmor<'source> {
    pub fn as_memo(&self) -> MortalArmorMemo {
        MortalArmorMemo {
            equipped: self.equipped.as_ref().map(|view| view.as_memo()),
            unequipped_mundane: self
                .unequipped_mundane
                .iter()
                .map(|(k, v)| (*k, v.as_memo()))
                .collect(),
            unequipped_artifact: self
                .unequipped_artifact
                .iter()
                .map(|(k, v)| (*k, v.as_memo()))
                .collect(),
        }
    }

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

    pub fn iter(&self) -> std::vec::IntoIter<ArmorId> {
        self.worn_armor()
            .iter()
            .map(|item| item.id())
            .chain(self.unequipped_mundane.keys().map(|k| ArmorId::Mundane(*k)))
            .chain(
                self.unequipped_artifact
                    .keys()
                    .map(|k| ArmorId::Artifact(*k)),
            )
            .collect::<Vec<ArmorId>>()
            .into_iter()
    }

    pub fn get(&self, armor_id: ArmorId) -> Option<ArmorItem<'source>> {
        let unequipped = match armor_id {
            ArmorId::Mundane(base_armor_id) => {
                self.unequipped_mundane
                    .get(&base_armor_id)
                    .map(|mundane_armor| {
                        ArmorItem(ArmorType::Mundane(base_armor_id, *mundane_armor), false)
                    })
            }
            ArmorId::Artifact(artifact_armor_id) => self
                .unequipped_artifact
                .get(&artifact_armor_id)
                .map(|no_attunement| {
                    ArmorItem(
                        ArmorType::Artifact(artifact_armor_id, no_attunement.clone(), None),
                        false,
                    )
                }),
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

    pub fn add_mundane(
        &mut self,
        armor_id: BaseArmorId,
        armor: &'source MundaneArmor,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self
            .worn_armor()
            .map_or(false, |item| item.id() == ArmorId::Mundane(armor_id))
        {
            Err(CharacterMutationError::ArmorError(
                ArmorError::DuplicateArmor,
            ))
        } else if let Entry::Vacant(e) = self.unequipped_mundane.entry(armor_id) {
            e.insert(armor.as_ref());
            Ok(self)
        } else {
            Err(CharacterMutationError::ArmorError(
                ArmorError::DuplicateArmor,
            ))
        }
    }

    pub fn remove_mundane(
        &mut self,
        armor_id: BaseArmorId,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.unequipped_mundane.remove(&armor_id).is_some() {
            Ok(self)
        } else if self
            .worn_armor()
            .map_or(false, |item| item.id() == ArmorId::Mundane(armor_id))
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

    pub fn equip(&mut self, armor_id: ArmorId) -> Result<&mut Self, CharacterMutationError> {
        let unstowed = match armor_id {
            ArmorId::Mundane(base_armor_id) => EquippedArmorNoAttunement::Mundane(
                base_armor_id,
                self.unequipped_mundane
                    .remove(&base_armor_id)
                    .ok_or(CharacterMutationError::ArmorError(ArmorError::NotFound))?,
            ),
            ArmorId::Artifact(artifact_armor_id) => EquippedArmorNoAttunement::Artifact(
                artifact_armor_id,
                self.unequipped_artifact
                    .remove(&artifact_armor_id)
                    .ok_or(CharacterMutationError::ArmorError(ArmorError::NotFound))?,
            ),
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
        armor_id: ArtifactArmorId,
        armor: ArtifactArmorView<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self
            .worn_armor()
            .map_or(false, |item| item.id() == ArmorId::Artifact(armor_id))
        {
            Err(CharacterMutationError::ArtifactError(
                ArtifactError::NamedArtifactsUnique,
            ))
        } else if let Entry::Vacant(e) = self.unequipped_artifact.entry(armor_id) {
            e.insert(armor.0);
            Ok(self)
        } else {
            Err(CharacterMutationError::ArmorError(
                ArmorError::DuplicateArmor,
            ))
        }
    }

    pub fn remove_artifact(
        &mut self,
        armor_id: ArtifactArmorId,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.unequipped_artifact.remove(&armor_id).is_some() {
            Ok(self)
        } else if self
            .worn_armor()
            .map_or(false, |item| item.id() == ArmorId::Artifact(armor_id))
        {
            Err(CharacterMutationError::ArmorError(
                ArmorError::RemoveEquipped,
            ))
        } else {
            Err(CharacterMutationError::ArmorError(ArmorError::NotFound))
        }
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
