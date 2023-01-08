use std::{collections::HashMap};

use crate::armor::armor_item::{BaseArmorId, artifact::{ArtifactArmorId, ArtifactArmorNoAttunement}, EquippedArmorNoAttunement, mundane::MundaneArmor, ArmorItem, ArmorType};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MortalArmor<'source> {
    equipped: Option<EquippedArmorNoAttunement<'source>>,
    unequipped_mundane: HashMap<BaseArmorId, MundaneArmor<'source>>,
    unequipped_artifact: HashMap<ArtifactArmorId, ArtifactArmorNoAttunement<'source>>,
}

impl<'source> MortalArmor<'source> {
    pub fn worn_armor(&self) -> Option<ArmorItem<'source>> {
        if let Some(equipped) = &self.equipped {
            match equipped {
                EquippedArmorNoAttunement::Mundane(id, mundane) => {
                    Some(ArmorItem(ArmorType::Mundane(*id, *mundane), true))
                }
                EquippedArmorNoAttunement::Artifact(id, artifact) => {
                    Some(ArmorItem(ArmorType::Artifact(*id, artifact.clone(), None), true))
                }
            }
        } else {
            None
        }
    }
}