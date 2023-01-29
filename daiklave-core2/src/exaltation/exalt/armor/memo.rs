use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::armor::armor_item::{artifact::ArtifactArmor, mundane::MundaneArmor, EquippedArmorMemo};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltArmorMemo {
    pub equipped: Option<EquippedArmorMemo>,
    pub unequipped_mundane: HashMap<String, MundaneArmor>,
    pub unequipped_artifact: HashMap<String, ArtifactArmor>,
}