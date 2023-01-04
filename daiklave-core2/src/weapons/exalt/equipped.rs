use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::weapons::{
    artifact::{
        ArtifactWeapon, HandlessArtifactWeapon, HandlessArtifactWeaponMemo,
        HandlessArtifactWeaponNoAttunement,
    },
    mortal::MortalEquippedWeapons,
    mundane::{HandlessMundaneWeapon, HandlessMundaneWeaponMemo, MundaneWeapon},
    ArtifactWeaponId, BaseWeaponId, Weapon, WeaponId, WeaponType,
};

use super::hands::{ExaltHands, ExaltHandsMemo};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(in crate::weapons) struct ExaltEquippedWeapons<'source> {
    pub handless_mundane: HashMap<BaseWeaponId, HandlessMundaneWeapon<'source>>,
    pub handless_artifact: HashMap<ArtifactWeaponId, HandlessArtifactWeapon<'source>>,
    pub hands: ExaltHands<'source>,
}

impl<'source> From<MortalEquippedWeapons<'source>> for ExaltEquippedWeapons<'source> {
    fn from(mortal: MortalEquippedWeapons) -> Self {
        Self {
            handless_mundane: mortal.handless_mundane,
            handless_artifact: mortal
                .handless_artifact
                .into_iter()
                .map(|(k, v)| (k, HandlessArtifactWeapon(v, None)))
                .collect(),
            hands: mortal.hands.into(),
        }
    }
}

impl<'source> ExaltEquippedWeapons<'source> {
    pub fn as_memo(&self) -> ExaltEquippedWeaponsMemo {
        ExaltEquippedWeaponsMemo {
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
                HandlessMundaneWeapon::Worn(worn_mundane) => Some(Weapon(WeaponType::Mundane(
                    target_id,
                    MundaneWeapon::Worn(*worn_mundane, true),
                ))),
            },
            WeaponId::Artifact(target_id) => {
                let handless_artifact_weapon = self.handless_artifact.get(&target_id)?;
                let (without_attunement, attunement) =
                    (handless_artifact_weapon.0, handless_artifact_weapon.1);

                match without_attunement {
                    HandlessArtifactWeaponNoAttunement::Natural(natural_artifact) => {
                        Some(Weapon(WeaponType::Artifact(
                            target_id,
                            ArtifactWeapon::Natural(natural_artifact),
                            attunement,
                        )))
                    }
                    HandlessArtifactWeaponNoAttunement::Worn(worn_artifact) => {
                        Some(Weapon(WeaponType::Artifact(
                            target_id,
                            ArtifactWeapon::Worn(worn_artifact, true),
                            attunement,
                        )))
                    }
                }
            }
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
