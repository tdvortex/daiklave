use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::HandlessArtifactWeaponNoAttunementMemo, mundane::HandlessMundaneWeaponMemo,
};

use super::hands::MortalHandsMemo;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalEquippedWeaponsMemo {
    pub handless_mundane: HashMap<String, HandlessMundaneWeaponMemo>,
    pub handless_artifact: HashMap<String, HandlessArtifactWeaponNoAttunementMemo>,
    pub hands: MortalHandsMemo,
}
