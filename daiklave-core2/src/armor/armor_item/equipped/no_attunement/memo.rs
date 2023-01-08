use serde::{Serialize, Deserialize};

use crate::armor::armor_item::{BaseArmorId, artifact::{ArtifactArmorId, ArtifactArmorNoAttunementMemo}, mundane::MundaneArmorMemo};

use super::EquippedArmorNoAttunement;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EquippedArmorNoAttunementMemo {
    Mundane(BaseArmorId, MundaneArmorMemo),
    Artifact(ArtifactArmorId, ArtifactArmorNoAttunementMemo),
}

impl<'source> EquippedArmorNoAttunementMemo {
    pub fn as_ref(&'source self) -> EquippedArmorNoAttunement<'source> {
        match self {
            EquippedArmorNoAttunementMemo::Mundane(base_armor_id, mundane_armor) => EquippedArmorNoAttunement::Mundane(*base_armor_id, mundane_armor.as_ref()),
            EquippedArmorNoAttunementMemo::Artifact(armor_artifact_id, artifact_armor) => EquippedArmorNoAttunement::Artifact(*armor_artifact_id, artifact_armor.as_ref()),
        }
    }
}