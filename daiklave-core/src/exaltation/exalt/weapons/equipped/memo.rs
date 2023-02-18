use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::HandlessArtifactWeaponMemo, mundane::HandlessMundaneWeaponMemo,
};

use super::{hands::ExaltHandsMemo, ExaltEquippedWeapons};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltEquippedWeaponsMemo {
    pub handless_mundane: HashMap<String, HandlessMundaneWeaponMemo>,
    pub handless_artifact: HashMap<String, HandlessArtifactWeaponMemo>,
    pub hands: ExaltHandsMemo,
}

impl From<&ExaltEquippedWeapons<'_>> for ExaltEquippedWeaponsMemo {
    fn from(value: &ExaltEquippedWeapons<'_>) -> Self {
        Self {
            handless_mundane: value.handless_mundane.iter().map(|(name, weapon)| ((*name).into(), weapon.into())).collect(),
            handless_artifact: value.handless_artifact.iter().map(|(name, weapon)| ((*name).into(), weapon.into())).collect(),
            hands: (&value.hands).into(),
        }
    }
}