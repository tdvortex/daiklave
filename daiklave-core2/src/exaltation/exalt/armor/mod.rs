use std::collections::HashMap;

use crate::armor::armor_item::{BaseArmorId, mundane::MundaneArmor, artifact::{ArtifactArmor, ArtifactArmorId}, ArmorItem, ArmorType, EquippedArmor};

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
                    Some(ArmorItem(ArmorType::Artifact(*id, no_attunement.clone(), attunement), true))
                }
            }
        } else {
            None
        }
    }
}