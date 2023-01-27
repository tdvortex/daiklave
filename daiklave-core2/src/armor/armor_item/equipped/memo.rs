use serde::{Deserialize, Serialize};

use crate::armor::armor_item::{
    artifact::{ArtifactArmor, ArtifactArmorId},
    mundane::MundaneArmor,
};

use super::EquippedArmor;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EquippedArmorMemo {
    Mundane(String, MundaneArmor),
    Artifact(ArtifactArmorId, ArtifactArmor),
}

impl<'source> EquippedArmorMemo {
    pub fn as_ref(&'source self) -> EquippedArmor<'source> {
        match self {
            EquippedArmorMemo::Mundane(name, memo) => {
                EquippedArmor::Mundane(name.as_str(), memo.as_ref())
            }
            EquippedArmorMemo::Artifact(id, memo) => EquippedArmor::Artifact(*id, memo.as_ref()),
        }
    }
}
