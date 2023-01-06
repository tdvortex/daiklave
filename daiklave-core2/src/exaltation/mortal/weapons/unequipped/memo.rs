use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{
    artifact::NonnaturalArtifactWeaponNoAttunementMemo, mundane::NonnaturalMundaneWeaponMemo,
    ArtifactWeaponId, BaseWeaponId,
};

use super::MortalUnequippedWeapons;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalUnequippedWeaponsMemo {
    pub mundane: HashMap<BaseWeaponId, NonnaturalMundaneWeaponMemo>,
    pub artifact: HashMap<ArtifactWeaponId, NonnaturalArtifactWeaponNoAttunementMemo>,
}

impl<'source> MortalUnequippedWeaponsMemo {
    pub fn as_ref(&'source self) -> MortalUnequippedWeapons<'source> {
        MortalUnequippedWeapons {
            mundane: self.mundane.iter().map(|(k, v)| (*k, v.as_ref())).collect(),
            artifact: self
                .artifact
                .iter()
                .map(|(k, v)| (*k, v.as_ref()))
                .collect(),
        }
    }
}
