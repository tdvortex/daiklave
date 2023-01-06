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
pub(crate) struct MortalEquippedWeapons<'source> {
    pub handless_mundane: HashMap<BaseWeaponId, HandlessMundaneWeapon<'source>>,
    pub handless_artifact: HashMap<ArtifactWeaponId, HandlessArtifactWeaponNoAttunement<'source>>,
    pub hands: MortalHands<'source>,
}

impl<'view, 'source> MortalEquippedWeapons<'source> {
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

    pub fn get_weapon(&'view self, weapon_id: WeaponId) -> Option<Weapon<'source>> {
        let in_hands = self.hands.get_weapon(weapon_id);
        if in_hands.is_some() {
            return in_hands;
        }

        match weapon_id {
            WeaponId::Unarmed => Some(crate::weapons::unarmed()),
            WeaponId::Mundane(target_id) => match self.handless_mundane.get(&target_id)? {
                HandlessMundaneWeapon::Natural(natural_mundane) => Some(Weapon(
                    WeaponType::Mundane(target_id, MundaneWeapon::Natural(natural_mundane.clone())),
                )),
                HandlessMundaneWeapon::Worn(worn) => Some(Weapon(WeaponType::Mundane(
                    target_id,
                    MundaneWeapon::Worn(worn.clone(), true),
                ))),
            },
            WeaponId::Artifact(target_id) => match self.handless_artifact.get(&target_id)? {
                HandlessArtifactWeaponNoAttunement::Natural(natural_artifact) => {
                    Some(Weapon(WeaponType::Artifact(
                        target_id,
                        ArtifactWeapon::Natural(natural_artifact.clone()),
                        None,
                    )))
                }
                HandlessArtifactWeaponNoAttunement::Worn(worn_artifact) => {
                    Some(Weapon(WeaponType::Artifact(
                        target_id,
                        ArtifactWeapon::Worn(worn_artifact.clone(), true),
                        None,
                    )))
                }
            },
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponId> + '_ {
        let unarmed_iter = std::iter::once(WeaponId::Unarmed);
        let handless_mundane_iter = self
            .handless_mundane
            .iter()
            .map(|(base_id, _)| WeaponId::Mundane(*base_id));
        let handless_artifact_iter = self
            .handless_artifact
            .iter()
            .map(|(artifact_id, _)| WeaponId::Artifact(*artifact_id));
        let hands_iter = self.hands.iter();

        unarmed_iter
            .chain(handless_artifact_iter)
            .chain(handless_mundane_iter)
            .chain(hands_iter)
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
pub(crate) struct MortalEquippedWeaponsMemo {
    handless_mundane: HashMap<BaseWeaponId, HandlessMundaneWeaponMemo>,
    handless_artifact: HashMap<ArtifactWeaponId, HandlessArtifactWeaponNoAttunementMemo>,
    hands: MortalHandsMemo,
}

impl<'source> MortalEquippedWeaponsMemo {
    pub fn as_ref(&'source self) -> MortalEquippedWeapons<'source> {
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
