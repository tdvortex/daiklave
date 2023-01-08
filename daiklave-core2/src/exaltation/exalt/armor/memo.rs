use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::armor::armor_item::{
    artifact::{ArtifactArmor, ArtifactArmorId},
    mundane::MundaneArmor,
    BaseArmorId, EquippedArmorMemo,
};

use super::ExaltArmor;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltArmorMemo {
    pub equipped: Option<EquippedArmorMemo>,
    pub unequipped_mundane: HashMap<BaseArmorId, MundaneArmor>,
    pub unequipped_artifact: HashMap<ArtifactArmorId, ArtifactArmor>,
}

impl<'source> ExaltArmorMemo {
    pub fn as_ref(&'source self) -> ExaltArmor<'source> {
        ExaltArmor {
            equipped: self.equipped.as_ref().map(|memo| memo.as_ref()),
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
