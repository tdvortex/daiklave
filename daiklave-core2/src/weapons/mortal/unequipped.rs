use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::weapons::{BaseWeaponId, ArtifactWeaponId, mundane::{NonnaturalMundaneWeapon, NonnaturalMundaneWeaponMemo}, artifact::{NonnaturalArtifactWeaponNoAttunement, NonnaturalArtifactWeaponNoAttunementMemo}, exalt::ExaltUnequippedWeapons};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(in crate::weapons) struct MortalUnequippedWeapons<'source> {
    pub mundane: HashMap<BaseWeaponId, NonnaturalMundaneWeapon<'source>>,
    pub artifact: HashMap<ArtifactWeaponId, NonnaturalArtifactWeaponNoAttunement<'source>>,
}

impl<'source> From<ExaltUnequippedWeapons<'source>> for MortalUnequippedWeapons<'source> {
    fn from(exalt: ExaltUnequippedWeapons<'source>) -> Self {
        Self {
            mundane: exalt.mundane,
            artifact: exalt.artifact.into_iter().map(|(k, v)| (k, v.0)).collect(),
        }
    }
}

impl<'source> MortalUnequippedWeapons<'source> {
    pub fn as_memo(&self) -> MortalUnequippedWeaponsMemo {
        MortalUnequippedWeaponsMemo {
            mundane: self.mundane.iter().map(|(k, v)| (*k, v.as_memo())).collect(),
            artifact: self.artifact.iter().map(|(k, v)| (*k, v.as_memo())).collect(),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons::mortal) struct MortalUnequippedWeaponsMemo {
    mundane: HashMap<BaseWeaponId, NonnaturalMundaneWeaponMemo>,
    artifact: HashMap<ArtifactWeaponId, NonnaturalArtifactWeaponNoAttunementMemo>,
}

impl<'source> MortalUnequippedWeaponsMemo {
    pub fn as_ref(&'source self) -> MortalUnequippedWeapons<'source> {
        MortalUnequippedWeapons {
            mundane: self.mundane.iter().map(|(k, v)| (*k, v.as_ref())).collect(),
            artifact: self.artifact.iter().map(|(k, v)| (*k, v.as_ref())).collect(),
        }
    }
}