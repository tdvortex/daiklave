use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::armor::armor_item::{
    artifact::{ArtifactArmorId, ArtifactArmorNoAttunementMemo},
    mundane::MundaneArmor,
    BaseArmorId, EquippedArmorNoAttunementMemo,
};

use super::MortalArmor;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalArmorMemo {
    pub equipped: Option<EquippedArmorNoAttunementMemo>,
    pub unequipped_mundane: HashMap<BaseArmorId, MundaneArmor>,
    pub unequipped_artifact: HashMap<ArtifactArmorId, ArtifactArmorNoAttunementMemo>,
}

impl<'source> MortalArmorMemo {
    pub fn as_ref(&'source self) -> MortalArmor<'source> {
        MortalArmor {
            equipped: self.equipped.as_ref().map(|equipped| equipped.as_ref()),
            unequipped_mundane: self
                .unequipped_mundane
                .iter()
                .map(|(k, v)| (*k, v.as_ref()))
                .collect(),
            unequipped_artifact: self
                .unequipped_artifact
                .iter()
                .map(|(k, v)| (*k, v.as_ref()))
                .collect(),
        }
    }
}
