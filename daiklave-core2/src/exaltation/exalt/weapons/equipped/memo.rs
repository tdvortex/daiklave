use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::HandlessArtifactWeaponMemo, mundane::HandlessMundaneWeaponMemo,
};

use super::hands::ExaltHandsMemo;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltEquippedWeaponsMemo {
    pub handless_mundane: HashMap<String, HandlessMundaneWeaponMemo>,
    pub handless_artifact: HashMap<String, HandlessArtifactWeaponMemo>,
    pub hands: ExaltHandsMemo,
}
