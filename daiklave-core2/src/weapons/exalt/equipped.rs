use std::collections::HashMap;

use crate::weapons::{BaseWeaponId, ArtifactWeaponId, mundane::HandlessMundaneWeapon, artifact::HandlessArtifactWeapon};

use super::hands::ExaltHands;

pub(in crate::weapons::exalt) struct ExaltEquippedWeapons<'source> {
    handless_mundane: HashMap<BaseWeaponId, HandlessMundaneWeapon<'source>>,
    handless_artifact: HashMap<ArtifactWeaponId, HandlessArtifactWeapon<'source>>,
    hands: ExaltHands<'source>,
}