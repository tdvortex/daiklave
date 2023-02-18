use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::armor::armor_item::{artifact::ArtifactArmor, mundane::MundaneArmor, EquippedArmorMemo};

use super::ExaltArmor;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltArmorMemo {
    pub equipped: Option<EquippedArmorMemo>,
    pub unequipped_mundane: HashMap<String, MundaneArmor>,
    pub unequipped_artifact: HashMap<String, ArtifactArmor>,
}

impl From<&ExaltArmor<'_>> for ExaltArmorMemo {
    fn from(value: &ExaltArmor<'_>) -> Self {
        Self {
            equipped: value.equipped.as_ref().map(|view| view.into()),
            unequipped_mundane: value
                .unequipped_mundane
                .iter()
                .map(|(name, view)| ((*name).into(), view.into()))
                .collect(),
            unequipped_artifact: value
                .unequipped_artifact
                .iter()
                .map(|(name, view)| ((*name).into(), view.into()))
                .collect(),
        }
    }
}
