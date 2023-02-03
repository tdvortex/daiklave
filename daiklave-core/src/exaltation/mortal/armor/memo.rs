use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::armor::armor_item::{
    artifact::ArtifactArmorNoAttunementMemo, mundane::MundaneArmor, EquippedArmorNoAttunementMemo,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalArmorMemo {
    pub equipped: Option<EquippedArmorNoAttunementMemo>,
    pub unequipped_mundane: HashMap<String, MundaneArmor>,
    pub unequipped_artifact: HashMap<String, ArtifactArmorNoAttunementMemo>,
}
