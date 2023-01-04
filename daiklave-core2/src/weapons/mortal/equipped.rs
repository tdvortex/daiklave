use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::weapons::{
    artifact::{
        ArtifactWeapon, HandlessArtifactWeaponNoAttunement, HandlessArtifactWeaponNoAttunementMemo,
    },
    exalt::ExaltEquippedWeapons,
    mundane::{HandlessMundaneWeapon, HandlessMundaneWeaponMemo, MundaneWeapon},
    ArtifactWeaponId, BaseWeaponId, Weapon, WeaponId, WeaponType,
};

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
            handless_mundane: self
                .handless_mundane
                .iter()
                .map(|(k, v)| (*k, v.as_memo()))
                .collect(),
            handless_artifact: self
                .handless_artifact
                .iter()
                .map(|(k, v)| (*k, v.as_memo()))
                .collect(),
            hands: self.hands.as_memo(),
        }
    }

    pub fn get_weapon(&self, weapon_id: WeaponId) -> Option<Weapon<'source>> {
        let in_hands = self.hands.get_weapon(weapon_id);
        if in_hands.is_some() {
            return in_hands;
        }

        match weapon_id {
            WeaponId::Unarmed => Some(crate::weapons::unarmed()),
            WeaponId::Mundane(target_id) => match self.handless_mundane.get(&target_id)? {
                HandlessMundaneWeapon::Natural(natural_mundane) => Some(Weapon(
                    WeaponType::Mundane(target_id, MundaneWeapon::Natural(*natural_mundane)),
                )),
                HandlessMundaneWeapon::Worn(worn) => Some(Weapon(WeaponType::Mundane(
                    target_id,
                    MundaneWeapon::Worn(*worn, true),
                ))),
            },
            WeaponId::Artifact(target_id) => match self.handless_artifact.get(&target_id)? {
                HandlessArtifactWeaponNoAttunement::Natural(natural_artifact) => {
                    Some(Weapon(WeaponType::Artifact(
                        target_id,
                        ArtifactWeapon::Natural(*natural_artifact),
                        None,
                    )))
                }
                HandlessArtifactWeaponNoAttunement::Worn(worn_artifact) => {
                    Some(Weapon(WeaponType::Artifact(
                        target_id,
                        ArtifactWeapon::Worn(*worn_artifact, true),
                        None,
                    )))
                }
            },
        }
    }
}

impl<'source> From<ExaltEquippedWeapons<'source>> for MortalEquippedWeapons<'source> {
    fn from(exalt: ExaltEquippedWeapons<'source>) -> Self {
        Self {
            handless_mundane: exalt.handless_mundane,
            handless_artifact: exalt
                .handless_artifact
                .into_iter()
                .map(|(k, v)| (k, v.0))
                .collect(),
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
            handless_mundane: self
                .handless_mundane
                .iter()
                .map(|(k, v)| (*k, v.as_ref()))
                .collect(),
            handless_artifact: self
                .handless_artifact
                .iter()
                .map(|(k, v)| (*k, v.as_ref()))
                .collect(),
            hands: self.hands.as_ref(),
        }
    }
}
