use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::armor::armor_item::{
    artifact::ArtifactArmorNoAttunementMemo, mundane::MundaneArmor, EquippedArmorNoAttunementMemo,
};

use super::MortalArmor;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalArmorMemo {
    pub equipped: Option<EquippedArmorNoAttunementMemo>,
    pub unequipped_mundane: HashMap<String, MundaneArmor>,
    pub unequipped_artifact: HashMap<String, ArtifactArmorNoAttunementMemo>,
}

impl From<&MortalArmor<'_>> for MortalArmorMemo {
    fn from(armor: &MortalArmor<'_>) -> Self {
        Self {
            equipped: armor.equipped.as_ref().map(|view| view.into()),
            unequipped_mundane: armor
                .unequipped_mundane
                .iter()
                .map(|(&name, view)| (name.into(), view.into()))
                .collect(),
            unequipped_artifact: armor
                .unequipped_artifact
                .iter()
                .map(|(&name, view)| (name.into(), view.into()))
                .collect(),
        }
    }
}
