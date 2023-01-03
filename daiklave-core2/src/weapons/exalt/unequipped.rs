use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::weapons::{BaseWeaponId, ArtifactWeaponId, mundane::{NonnaturalMundaneWeapon, NonnaturalMundaneWeaponMemo}, artifact::{NonnaturalArtifactWeapon, NonnaturalArtifactWeaponMemo}};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(in crate::weapons::exalt) struct ExaltUnequippedWeapons<'source> {
    mundane: HashMap<BaseWeaponId, NonnaturalMundaneWeapon<'source>>,
    artifact: HashMap<ArtifactWeaponId, NonnaturalArtifactWeapon<'source>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons::exalt) struct ExaltUnequippedWeaponsMemo {
    mundane: HashMap<BaseWeaponId, NonnaturalMundaneWeaponMemo>,
    artifact: HashMap<ArtifactWeaponId, NonnaturalArtifactWeaponMemo>,
}