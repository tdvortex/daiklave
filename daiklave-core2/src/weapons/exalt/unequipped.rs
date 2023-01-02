use std::collections::HashMap;

use crate::weapons::{BaseWeaponId, ArtifactWeaponId, mundane::NonnaturalMundaneWeapon, artifact::NonnaturalArtifactWeapon};

pub(in crate::weapons::exalt) struct ExaltUnequippedWeapons<'source> {
    mundane: HashMap<BaseWeaponId, NonnaturalMundaneWeapon<'source>>,
    artifact: HashMap<ArtifactWeaponId, NonnaturalArtifactWeapon<'source>>,
}