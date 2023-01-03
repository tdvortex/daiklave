use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::weapons::{BaseWeaponId, ArtifactWeaponId, artifact::{HandlessArtifactWeaponNoAttunement, HandlessArtifactWeaponNoAttunementMemo}, mundane::{HandlessMundaneWeapon, HandlessMundaneWeaponMemo}, exalt::ExaltEquippedWeapons};

use super::hands::{MortalHands, MortalHandsMemo};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(in crate::weapons) struct MortalEquippedWeapons<'source> {
    pub handless_mundane: HashMap<BaseWeaponId, HandlessMundaneWeapon<'source>>,
    pub handless_artifact: HashMap<ArtifactWeaponId, HandlessArtifactWeaponNoAttunement<'source>>,
    pub hands: MortalHands<'source>,
}

impl<'source> MortalEquippedWeapons<'source> {
    pub fn as_memo(&self) -> MortalEquippedWeaponsMemo {
        MortalEquippedWeaponsMemo { 
            handless_mundane: self.handless_mundane.iter().map(|(k, v)| (*k, v.as_memo())).collect(),
            handless_artifact: self.handless_artifact.iter().map(|(k, v)| (*k, v.as_memo())).collect(), 
            hands: self.hands.as_memo()
        }
    }
}

impl<'source> From<ExaltEquippedWeapons<'source>> for MortalEquippedWeapons<'source> {
    fn from(exalt: ExaltEquippedWeapons<'source>) -> Self {
        Self {
            handless_mundane: exalt.handless_mundane,
            handless_artifact: exalt.handless_artifact.into_iter().map(|(k, v)| (k, v.0)).collect(),
            hands: exalt.hands.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons::mortal) struct MortalEquippedWeaponsMemo {
    handless_mundane: HashMap<BaseWeaponId, HandlessMundaneWeaponMemo>,
    handless_artifact: HashMap<ArtifactWeaponId, HandlessArtifactWeaponNoAttunementMemo>,
    hands: MortalHandsMemo,
}

impl<'source> MortalEquippedWeaponsMemo {
    pub fn as_ref(&self) -> MortalEquippedWeapons<'source> {
        MortalEquippedWeapons { 
            handless_mundane: self.handless_mundane.iter().map(|(k, v)| (*k, v.as_ref())).collect(),
            handless_artifact: self.handless_artifact.iter().map(|(k, v)| (*k, v.as_ref())).collect(), 
            hands: self.hands.as_ref()
        }
    }
}