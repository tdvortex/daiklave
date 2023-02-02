use serde::{Deserialize, Serialize};

use crate::armor::armor_item::{
    artifact::{ArtifactArmorName, ArtifactArmorNoAttunementMemo},
    mundane::{MundaneArmor, MundaneArmorName},
};

use super::EquippedArmorNoAttunement;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EquippedArmorNoAttunementMemo {
    Mundane(MundaneArmorName, MundaneArmor),
    Artifact(ArtifactArmorName, ArtifactArmorNoAttunementMemo),
}

impl From<&EquippedArmorNoAttunement<'_>> for EquippedArmorNoAttunementMemo {
    fn from(view: &EquippedArmorNoAttunement<'_>) -> Self {
        match view {
            EquippedArmorNoAttunement::Mundane(name, mundane_armor) => {
                EquippedArmorNoAttunementMemo::Mundane((*name).into(), mundane_armor.into())
            }
            EquippedArmorNoAttunement::Artifact(name, artifact_armor) => {
                EquippedArmorNoAttunementMemo::Artifact((*name).into(), artifact_armor.into())
            }
        }
    }
}
