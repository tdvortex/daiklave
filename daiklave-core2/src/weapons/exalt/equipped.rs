use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::weapons::{BaseWeaponId, ArtifactWeaponId, mundane::{HandlessMundaneWeapon, HandlessMundaneWeaponMemo}, artifact::{HandlessArtifactWeapon, HandlessArtifactWeaponMemo}};

use super::hands::{ExaltHands, ExaltHandsMemo};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(in crate::weapons::exalt) struct ExaltEquippedWeapons<'source> {
    handless_mundane: HashMap<BaseWeaponId, HandlessMundaneWeapon<'source>>,
    handless_artifact: HashMap<ArtifactWeaponId, HandlessArtifactWeapon<'source>>,
    hands: ExaltHands<'source>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons::exalt) struct ExaltEquippedWeaponsMemo {
    handless_mundane: HashMap<BaseWeaponId, HandlessMundaneWeaponMemo>,
    handless_artifact: HashMap<ArtifactWeaponId, HandlessArtifactWeaponMemo>,
    hands: ExaltHandsMemo,
}

impl<'source> ExaltEquippedWeaponsMemo {
    pub fn as_ref(&'source self) -> ExaltEquippedWeapons<'source> {
        ExaltEquippedWeapons {
            handless_mundane: self.handless_mundane.iter().map(|(k, v)| (*k, v.as_ref())).collect(),
            handless_artifact: self.handless_artifact.iter().map(|(k, v)| (*k, v.as_ref())).collect(),
            hands: self.hands.as_ref(),
        }
    }
}