use serde::{Serialize, Deserialize};

use crate::armor::armor_item::{BaseArmorId, mundane::MundaneArmorMemo, artifact::{ArtifactArmorId, ArtifactArmorMemo}};

use super::EquippedArmor;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum EquippedArmorMemo {
    Mundane(BaseArmorId, MundaneArmorMemo),
    Artifact(ArtifactArmorId, ArtifactArmorMemo),
}

impl<'source> EquippedArmorMemo {
    pub fn as_ref(&'source self) -> EquippedArmor<'source> {
        match self {
            EquippedArmorMemo::Mundane(id, memo) => EquippedArmor::Mundane(*id, memo.as_ref()),
            EquippedArmorMemo::Artifact(id, memo) => EquippedArmor::Artifact(*id, memo.as_ref()),
        }
    }
}