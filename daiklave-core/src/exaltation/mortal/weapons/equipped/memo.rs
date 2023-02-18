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

impl From<&MortalEquippedWeapons<'_>> for MortalEquippedWeaponsMemo {
    fn from(weapons: &MortalEquippedWeapons<'_>) -> Self {
        Self {
            handless_mundane: weapons
                .handless_mundane
                .iter()
                .map(|(&name, weapon)| (name.into(), weapon.into()))
                .collect(),
            handless_artifact: weapons
                .handless_artifact
                .iter()
                .map(|(&name, weapon)| (name.into(), weapon.into()))
                .collect(),
            hands: (&weapons.hands).into(),
        }
    }
}
