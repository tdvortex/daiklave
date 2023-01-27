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

impl<'source> ExaltUnequippedWeaponsMemo {
    pub fn as_ref(&'source self) -> ExaltUnequippedWeapons<'source> {
        ExaltUnequippedWeapons {
            mundane: self
                .mundane
                .iter()
                .map(|(k, (v, count))| (k.as_str(), (v.as_ref(), *count)))
                .collect(),
            artifact: self
                .artifact
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_ref()))
                .collect(),
        }
    }
}
