use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::weapons::{BaseWeaponId, ArtifactWeaponId, mundane::{HandlessMundaneWeapon, HandlessMundaneWeaponMemo}, artifact::{HandlessArtifactWeapon, HandlessArtifactWeaponMemo}, mortal::MortalEquippedWeapons};

use super::hands::{ExaltHands, ExaltHandsMemo};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(in crate::weapons::exalt) struct ExaltEquippedWeapons<'source> {
    handless_mundane: HashMap<BaseWeaponId, HandlessMundaneWeapon<'source>>,
    handless_artifact: HashMap<ArtifactWeaponId, HandlessArtifactWeapon<'source>>,
    hands: ExaltHands<'source>,
}

impl<'source> From<MortalEquippedWeapons<'source>> for ExaltEquippedWeapons<'source> {
    fn from(mortal: MortalEquippedWeapons) -> Self {
        Self {
            handless_mundane: mortal.handless_mundane,
            handless_artifact: mortal.handless_artifact.into_iter().map(|(k, v)| (k, HandlessArtifactWeapon(v, None))).collect(),
            hands: mortal.hands.into(),
        }
    }
}

impl<'source> ExaltEquippedWeapons<'source> {
    pub fn as_memo(&self) -> ExaltEquippedWeaponsMemo {
        ExaltEquippedWeaponsMemo {
            handless_mundane: self.handless_mundane.iter().map(|(k, v)| (*k, v.as_memo())).collect(),
            handless_artifact: self.handless_artifact.iter().map(|(k, v)| (*k, v.as_memo())).collect(),
            hands: self.hands.as_memo(),
        }
    }
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