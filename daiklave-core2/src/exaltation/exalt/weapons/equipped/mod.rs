use std::collections::{hash_map::Entry, HashMap};

use crate::{
    exaltation::mortal::MortalEquippedWeapons,
    weapons::{
        weapon::{
            artifact::{
                ArtifactWeapon, HandlessArtifactWeapon, HandlessArtifactWeaponNoAttunement,
            },
            mundane::{HandlessMundaneWeapon, MundaneWeapon},
            ArtifactWeaponId, BaseWeaponId, Weapon, WeaponId, WeaponType, Equipped,
        },
        WeaponError,
    },
    CharacterMutationError,
};

mod hands;
mod memo;
pub use hands::ExaltHands;
pub(crate) use memo::ExaltEquippedWeaponsMemo;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ExaltEquippedWeapons<'source> {
    pub handless_mundane: HashMap<BaseWeaponId, HandlessMundaneWeapon<'source>>,
    pub handless_artifact: HashMap<ArtifactWeaponId, HandlessArtifactWeapon<'source>>,
    pub hands: ExaltHands<'source>,
}

impl<'source> From<MortalEquippedWeapons<'source>> for ExaltEquippedWeapons<'source> {
    fn from(mortal: MortalEquippedWeapons<'source>) -> Self {
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

impl<'view, 'source> ExaltEquippedWeapons<'source> {
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

    pub fn get_weapon(&'view self, weapon_id: WeaponId) -> Option<Weapon<'source>> {
        let in_hands = self.hands.get_weapon(weapon_id);
        if in_hands.is_some() {
            return in_hands;
        }

        match weapon_id {
            WeaponId::Unarmed => Some(crate::weapons::weapon::mundane::unarmed()),
            WeaponId::Mundane(target_id) => match self.handless_mundane.get(&target_id)? {
                HandlessMundaneWeapon::Natural(natural_mundane) => Some(Weapon(
                    WeaponType::Mundane(target_id, MundaneWeapon::Natural(natural_mundane.clone())),
                )),
                HandlessMundaneWeapon::Worn(worn_mundane) => Some(Weapon(WeaponType::Mundane(
                    target_id,
                    MundaneWeapon::Worn(worn_mundane.clone(), true),
                ))),
            },
            WeaponId::Artifact(target_id) => {
                let handless_artifact_weapon = self.handless_artifact.get(&target_id)?;
                let (without_attunement, attunement) =
                    (&handless_artifact_weapon.0, handless_artifact_weapon.1);

                match without_attunement {
                    HandlessArtifactWeaponNoAttunement::Natural(natural_artifact) => {
                        Some(Weapon(WeaponType::Artifact(
                            target_id,
                            ArtifactWeapon::Natural(natural_artifact.clone()),
                            attunement,
                        )))
                    }
                    HandlessArtifactWeaponNoAttunement::Worn(worn_artifact) => {
                        Some(Weapon(WeaponType::Artifact(
                            target_id,
                            ArtifactWeapon::Worn(worn_artifact.clone(), true),
                            attunement,
                        )))
                    }
                }
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (WeaponId, Option<Equipped>)> + '_ {
        std::iter::once((WeaponId::Unarmed, Some(Equipped::Natural)))
            .chain(self.hands.iter())
            .chain(
                self.handless_mundane
                    .iter()
                    .map(|(base_id, weapon)| {
                        (WeaponId::Mundane(*base_id), match weapon {
                            HandlessMundaneWeapon::Natural(_) => Some(Equipped::Natural),
                            HandlessMundaneWeapon::Worn(_) => Some(Equipped::Worn),
                        })
                    }),
            )
            .chain(
                self.handless_artifact
                    .iter()
                    .map(|(artifact_id, weapon)| {
                        (WeaponId::Artifact(*artifact_id), match weapon {
                            HandlessArtifactWeapon(HandlessArtifactWeaponNoAttunement::Natural(_), _) => Some(Equipped::Natural),
                            HandlessArtifactWeapon(HandlessArtifactWeaponNoAttunement::Worn(_), _) => Some(Equipped::Worn),
                        })
                    }),
            )
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
