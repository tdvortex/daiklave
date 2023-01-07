use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::NonnaturalArtifactWeaponMemo, mundane::NonnaturalMundaneWeaponMemo, ArtifactWeaponId,
    BaseWeaponId,
};

use super::ExaltUnequippedWeapons;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltUnequippedWeaponsMemo {
    pub mundane: HashMap<BaseWeaponId, (NonnaturalMundaneWeaponMemo, u8)>,
    pub artifact: HashMap<ArtifactWeaponId, NonnaturalArtifactWeaponMemo>,
}

impl<'source> ExaltUnequippedWeaponsMemo {
    pub fn as_ref(&'source self) -> ExaltUnequippedWeapons<'source> {
        ExaltUnequippedWeapons {
            mundane: self
                .mundane
                .iter()
                .map(|(k, (v, count))| (*k, (v.as_ref(), *count)))
                .collect(),
            artifact: self
                .artifact
                .iter()
                .map(|(k, v)| (*k, v.as_ref()))
                .collect(),
        }
    }
}
