use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::weapons::{BaseWeaponId, ArtifactWeaponId, mundane::{NonnaturalMundaneWeapon, NonnaturalMundaneWeaponMemo}, artifact::{NonnaturalArtifactWeapon, NonnaturalArtifactWeaponMemo}, mortal::MortalUnequippedWeapons};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(in crate::weapons::exalt) struct ExaltUnequippedWeapons<'source> {
    mundane: HashMap<BaseWeaponId, NonnaturalMundaneWeapon<'source>>,
    artifact: HashMap<ArtifactWeaponId, NonnaturalArtifactWeapon<'source>>,
}

impl<'source> From<MortalUnequippedWeapons<'source>> for ExaltUnequippedWeapons<'source> {
    fn from(mortal: MortalUnequippedWeapons<'source>) -> Self {
        Self {
            mundane: mortal.mundane,
            artifact: mortal.artifact.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}

impl<'source> ExaltUnequippedWeapons<'source> {
    pub fn as_memo(&self) -> ExaltUnequippedWeaponsMemo {
        ExaltUnequippedWeaponsMemo {
            mundane: self.mundane.iter().map(|(k, v)| (*k, v.as_memo())).collect(),
            artifact: self.artifact.iter().map(|(k, v)| (*k, v.as_memo())).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons::exalt) struct ExaltUnequippedWeaponsMemo {
    mundane: HashMap<BaseWeaponId, NonnaturalMundaneWeaponMemo>,
    artifact: HashMap<ArtifactWeaponId, NonnaturalArtifactWeaponMemo>,
}

impl<'source> ExaltUnequippedWeaponsMemo {
    pub fn as_ref(&'source self) -> ExaltUnequippedWeapons<'source> {
        ExaltUnequippedWeapons {
            mundane: self.mundane.iter().map(|(k, v)| (*k, v.as_ref())).collect(),
            artifact: self.artifact.iter().map(|(k, v)| (*k, v.as_ref())).collect(),
        }
    }
}