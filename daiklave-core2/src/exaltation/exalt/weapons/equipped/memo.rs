use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::HandlessArtifactWeaponMemo, mundane::HandlessMundaneWeaponMemo, ArtifactWeaponId,
};

use super::{hands::ExaltHandsMemo, ExaltEquippedWeapons};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltEquippedWeaponsMemo {
    pub handless_mundane: HashMap<String, HandlessMundaneWeaponMemo>,
    pub handless_artifact: HashMap<ArtifactWeaponId, HandlessArtifactWeaponMemo>,
    pub hands: ExaltHandsMemo,
}

impl<'source> ExaltEquippedWeaponsMemo {
    pub fn as_ref(&'source self) -> ExaltEquippedWeapons<'source> {
        ExaltEquippedWeapons {
            handless_mundane: self
                .handless_mundane
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_ref()))
                .collect(),
            handless_artifact: self
                .handless_artifact
                .iter()
                .map(|(k, v)| (*k, v.as_ref()))
                .collect(),
            hands: self.hands.as_ref(),
        }
    }
}
