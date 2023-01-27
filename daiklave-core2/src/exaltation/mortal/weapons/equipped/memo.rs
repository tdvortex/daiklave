use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::HandlessArtifactWeaponNoAttunementMemo, mundane::HandlessMundaneWeaponMemo,
};

use super::{hands::MortalHandsMemo, MortalEquippedWeapons};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalEquippedWeaponsMemo {
    pub handless_mundane: HashMap<String, HandlessMundaneWeaponMemo>,
    pub handless_artifact: HashMap<String, HandlessArtifactWeaponNoAttunementMemo>,
    pub hands: MortalHandsMemo,
}

impl<'source> MortalEquippedWeaponsMemo {
    pub fn as_ref(&'source self) -> MortalEquippedWeapons<'source> {
        MortalEquippedWeapons {
            handless_mundane: self
                .handless_mundane
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_ref()))
                .collect(),
            handless_artifact: self
                .handless_artifact
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_ref()))
                .collect(),
            hands: self.hands.as_ref(),
        }
    }
}
