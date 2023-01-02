use std::collections::HashMap;

use crate::weapons::{BaseWeaponId, ArtifactWeaponId, artifact::HandlessArtifactWeaponNoAttunement, mundane::HandlessMundaneWeapon};

use super::hands::MortalHands;

pub(in crate::weapons::mortal) struct MortalEquippedWeapons<'source> {
    handless_mundane: HashMap<BaseWeaponId, HandlessMundaneWeapon<'source>>,
    handless_artifact: HashMap<ArtifactWeaponId, HandlessArtifactWeaponNoAttunement<'source>>,
    hands: MortalHands<'source>,
}