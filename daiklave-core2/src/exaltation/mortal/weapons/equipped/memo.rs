use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::HandlessArtifactWeaponNoAttunementMemo, mundane::HandlessMundaneWeaponMemo,
    ArtifactWeaponId, BaseWeaponId,
};

use super::{hands::MortalHandsMemo, MortalEquippedWeapons};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalEquippedWeaponsMemo {
    pub handless_mundane: HashMap<BaseWeaponId, HandlessMundaneWeaponMemo>,
    pub handless_artifact: HashMap<ArtifactWeaponId, HandlessArtifactWeaponNoAttunementMemo>,
    pub hands: MortalHandsMemo,
}

impl<'source> MortalEquippedWeaponsMemo {
    pub fn as_ref(&'source self) -> MortalEquippedWeapons<'source> {
        MortalEquippedWeapons {
            handless_mundane: self
                .handless_mundane
                .iter()
                .map(|(k, v)| (*k, v.as_ref()))
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
