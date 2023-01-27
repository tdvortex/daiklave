use serde::{Deserialize, Serialize};

use crate::armor::armor_item::{artifact::ArtifactArmorNoAttunementMemo, mundane::MundaneArmor};

use super::EquippedArmorNoAttunement;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EquippedArmorNoAttunementMemo {
    Mundane(String, MundaneArmor),
    Artifact(String, ArtifactArmorNoAttunementMemo),
}

impl<'source> EquippedArmorNoAttunementMemo {
    pub fn as_ref(&'source self) -> EquippedArmorNoAttunement<'source> {
        match self {
            EquippedArmorNoAttunementMemo::Mundane(name, mundane_armor) => {
                EquippedArmorNoAttunement::Mundane(name.as_str(), mundane_armor.as_ref())
            }
            EquippedArmorNoAttunementMemo::Artifact(name, artifact_armor) => {
                EquippedArmorNoAttunement::Artifact(name.as_str(), artifact_armor.as_ref())
            }
        }
    }
}
