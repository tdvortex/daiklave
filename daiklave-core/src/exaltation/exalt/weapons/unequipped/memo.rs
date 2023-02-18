use std::{collections::HashMap, num::NonZeroU8};

use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::NonnaturalArtifactWeaponMemo, mundane::NonnaturalMundaneWeaponMemo,
};

use super::ExaltUnequippedWeapons;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltUnequippedWeaponsMemo {
    pub mundane: HashMap<String, (NonnaturalMundaneWeaponMemo, NonZeroU8)>,
    pub artifact: HashMap<String, NonnaturalArtifactWeaponMemo>,
}

impl From<&ExaltUnequippedWeapons<'_>> for ExaltUnequippedWeaponsMemo {
    fn from(value: &ExaltUnequippedWeapons<'_>) -> Self {
        Self {
            mundane: value
                .mundane
                .iter()
                .map(|(name, (weapon, quantity))| ((*name).into(), (weapon.into(), *quantity)))
                .collect(),
            artifact: value
                .artifact
                .iter()
                .map(|(name, weapon)| ((*name).into(), weapon.into()))
                .collect(),
        }
    }
}
