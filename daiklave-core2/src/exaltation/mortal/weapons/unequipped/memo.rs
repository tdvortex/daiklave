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

impl<'source> MortalUnequippedWeaponsMemo {
    pub fn as_ref(&'source self) -> MortalUnequippedWeapons<'source> {
        MortalUnequippedWeapons {
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
