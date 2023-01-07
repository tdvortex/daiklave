mod hands;
mod memo;
pub(crate) use hands::MortalHands;
pub(crate) use memo::MortalEquippedWeaponsMemo;

use std::{collections::{hash_map::Entry, HashMap}};

use crate::{
    exaltation::exalt::ExaltEquippedWeapons,
    weapons::{
        weapon::{
            artifact::{ArtifactWeapon, HandlessArtifactWeaponNoAttunement},
            mundane::{HandlessMundaneWeapon, MundaneWeapon},
            ArtifactWeaponId, BaseWeaponId, Weapon, WeaponId, WeaponType, Equipped,
        },
        WeaponError,
    },
    CharacterMutationError,
};

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

    pub fn get_weapon(&'view self, weapon_id: WeaponId, equipped: Equipped) -> Option<Weapon<'source>> {
        match (weapon_id, equipped) {
            (WeaponId::Unarmed, Equipped::Natural) => Some(crate::weapons::weapon::mundane::unarmed()),
            (WeaponId::Unarmed, _) => None,
            (WeaponId::Mundane(base_weapon_id), Equipped::Natural) => {
                match self.handless_mundane.get(&base_weapon_id)? {
                    HandlessMundaneWeapon::Natural(weapon) => Some(Weapon(
                        WeaponType::Mundane(base_weapon_id, MundaneWeapon::Natural(weapon.clone())),
                    )),
                    HandlessMundaneWeapon::Worn(_) => None,
                }
            }
            (WeaponId::Mundane(base_weapon_id), Equipped::Worn) => {
                match self.handless_mundane.get(&base_weapon_id)? {
                    HandlessMundaneWeapon::Worn(weapon) => Some(Weapon(
                        WeaponType::Mundane(base_weapon_id, MundaneWeapon::Worn(weapon.clone(), true)),
                    )),
                    HandlessMundaneWeapon::Natural(_) => None,
                }
            }
            (WeaponId::Artifact(artifact_weapon_id), Equipped::Natural) => {
                match self.handless_artifact.get(&artifact_weapon_id)? {
                    HandlessArtifactWeaponNoAttunement::Natural(weapon) => Some(
                        Weapon(
                            WeaponType::Artifact(artifact_weapon_id, ArtifactWeapon::Natural(weapon.clone()), None)
                        )
                    ),
                    HandlessArtifactWeaponNoAttunement::Worn(_) => None,
                }
            }
            (WeaponId::Artifact(artifact_weapon_id), Equipped::Worn) => {
                match self.handless_artifact.get(&artifact_weapon_id)? {
                    HandlessArtifactWeaponNoAttunement::Worn(weapon) => Some(
                        Weapon(
                            WeaponType::Artifact(artifact_weapon_id, ArtifactWeapon::Worn(weapon.clone(), true), None)
                        )
                    ),
                    HandlessArtifactWeaponNoAttunement::Natural(_) => None,
                }
            }
            (_, equipped) => self.hands.get_weapon(weapon_id, equipped)
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (WeaponId, Option<Equipped>)> + '_ {
        let unarmed_iter = std::iter::once((WeaponId::Unarmed, Some(Equipped::Worn)));
        let handless_mundane_iter = self
            .handless_mundane
            .iter()
            .map(|(base_id, weapon)| {
                (WeaponId::Mundane(*base_id),
                match weapon {
                    HandlessMundaneWeapon::Natural(_) => Some(Equipped::Natural),
                    HandlessMundaneWeapon::Worn(_) => Some(Equipped::Worn),
                })
            });
        let handless_artifact_iter = self
            .handless_artifact
            .iter()
            .map(|(artifact_id, weapon)| {
                (WeaponId::Artifact(*artifact_id),
                match weapon {
                    HandlessArtifactWeaponNoAttunement::Natural(_) => Some(Equipped::Natural),
                    HandlessArtifactWeaponNoAttunement::Worn(_) => Some(Equipped::Worn),
                })
            });
        let hands_iter = self.hands.iter();

        unarmed_iter
            .chain(handless_artifact_iter)
            .chain(handless_mundane_iter)
            .chain(hands_iter)
    }

    pub fn add_natural_mundane_weapon(
        &mut self,
        weapon_id: BaseWeaponId,
        weapon: HandlessMundaneWeapon<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Entry::Vacant(e) = self.handless_mundane.entry(weapon_id) {
            e.insert(weapon);
            Ok(self)
        } else {
            Err(CharacterMutationError::WeaponError(
                WeaponError::DuplicateNatural,
            ))
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
