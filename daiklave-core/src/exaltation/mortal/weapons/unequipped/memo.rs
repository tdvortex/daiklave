use std::{collections::HashMap, num::NonZeroU8};

use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::NonnaturalArtifactWeaponNoAttunementMemo, mundane::NonnaturalMundaneWeaponMemo,
};

use super::MortalUnequippedWeapons;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalUnequippedWeaponsMemo {
    pub mundane: HashMap<String, (NonnaturalMundaneWeaponMemo, NonZeroU8)>,
    pub artifact: HashMap<String, NonnaturalArtifactWeaponNoAttunementMemo>,
}

impl From<&MortalUnequippedWeapons<'_>> for MortalUnequippedWeaponsMemo {
    fn from(value: &MortalUnequippedWeapons<'_>) -> Self {
        Self {
            mundane: value
                .mundane
                .iter()
                .map(|(name, (weapon, quantity))| ((*name).into(), (weapon.into(), *quantity)))
                .collect(),
            artifact: value
                .artifact
                .iter()
                .map(|(name, artifact)| ((*name).into(), artifact.into()))
                .collect(),
        }
    }
}
