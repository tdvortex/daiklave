use serde::{Deserialize, Serialize};

use crate::armor::armor_item::{artifact::{ArtifactArmor, ArtifactArmorName}, mundane::{MundaneArmor, MundaneArmorName}};

use super::EquippedArmor;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EquippedArmorMemo {
    Mundane(MundaneArmorName, MundaneArmor),
    Artifact(ArtifactArmorName, ArtifactArmor),
}

impl From<&EquippedArmor<'_>> for EquippedArmorMemo {
    fn from(view: &EquippedArmor<'_>) -> Self {
        match view {
            EquippedArmor::Mundane(name, view) => {
                EquippedArmorMemo::Mundane((*name).into(), view.into())
            }
            EquippedArmor::Artifact(name, view) => {
                EquippedArmorMemo::Artifact((*name).into(), view.into())
            }
        }
    }
}