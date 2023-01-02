use std::collections::HashMap;

use crate::weapons::{BaseWeaponId, ArtifactWeaponId, mundane::NonnaturalMundaneWeapon, artifact::NonnaturalArtifactWeaponNoAttunement};

pub(in crate::weapons::mortal) struct MortalUnequippedWeapons<'source> {
    mundane: HashMap<BaseWeaponId, NonnaturalMundaneWeapon<'source>>,
    artifact: HashMap<ArtifactWeaponId, NonnaturalArtifactWeaponNoAttunement<'source>>,
}