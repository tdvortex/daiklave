use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::weapons::{
    artifact::{
        ArtifactWeapon, NonnaturalArtifactWeapon, NonnaturalArtifactWeaponMemo,
        NonnaturalArtifactWeaponNoAttunement,
    },
    mortal::MortalUnequippedWeapons,
    mundane::{MundaneWeapon, NonnaturalMundaneWeapon, NonnaturalMundaneWeaponMemo},
    ArtifactWeaponId, BaseWeaponId, Weapon, WeaponId, WeaponType,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(in crate::weapons) struct ExaltUnequippedWeapons<'source> {
    pub mundane: HashMap<BaseWeaponId, NonnaturalMundaneWeapon<'source>>,
    pub artifact: HashMap<ArtifactWeaponId, NonnaturalArtifactWeapon<'source>>,
}

impl<'source> From<MortalUnequippedWeapons<'source>> for ExaltUnequippedWeapons<'source> {
    fn from(mortal: MortalUnequippedWeapons<'source>) -> Self {
        Self {
            mundane: mortal.mundane,
            artifact: mortal
                .artifact
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl<'view, 'source> ExaltUnequippedWeapons<'source> {
    pub fn as_memo(&self) -> ExaltUnequippedWeaponsMemo {
        ExaltUnequippedWeaponsMemo {
            mundane: self
                .mundane
                .iter()
                .map(|(k, v)| (*k, v.as_memo()))
                .collect(),
            artifact: self
                .artifact
                .iter()
                .map(|(k, v)| (*k, v.as_memo()))
                .collect(),
        }
    }

    pub fn get_weapon(&'view self, weapon_id: WeaponId) -> Option<Weapon<'view, 'source>> {
        match weapon_id {
            WeaponId::Unarmed => Some(crate::weapons::unarmed()),
            WeaponId::Mundane(target_id) => match self.mundane.get(&target_id)? {
                NonnaturalMundaneWeapon::Worn(worn_weapon) => Some(Weapon(WeaponType::Mundane(
                    target_id,
                    MundaneWeapon::Worn(*worn_weapon, false),
                ))),
                NonnaturalMundaneWeapon::OneHanded(one) => Some(Weapon(WeaponType::Mundane(
                    target_id,
                    MundaneWeapon::OneHanded(*one, None),
                ))),
                NonnaturalMundaneWeapon::TwoHanded(two) => Some(Weapon(WeaponType::Mundane(
                    target_id,
                    MundaneWeapon::TwoHanded(*two, false),
                ))),
            },
            WeaponId::Artifact(target_id) => {
                let nonnatural_artifact_weapon = self.artifact.get(&target_id)?;
                let (without_attunement, attunement) =
                    (&nonnatural_artifact_weapon.0, nonnatural_artifact_weapon.1);

                match without_attunement {
                    NonnaturalArtifactWeaponNoAttunement::Worn(worn) => {
                        Some(Weapon(WeaponType::Artifact(
                            target_id,
                            ArtifactWeapon::Worn(worn, false),
                            attunement,
                        )))
                    }
                    NonnaturalArtifactWeaponNoAttunement::OneHanded(one) => {
                        Some(Weapon(WeaponType::Artifact(
                            target_id,
                            ArtifactWeapon::OneHanded(one, None),
                            attunement,
                        )))
                    }
                    NonnaturalArtifactWeaponNoAttunement::TwoHanded(two) => {
                        Some(Weapon(WeaponType::Artifact(
                            target_id,
                            ArtifactWeapon::TwoHanded(two, false),
                            attunement,
                        )))
                    }
                }
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponId> + '_ {
        self.mundane
            .iter()
            .map(|(base_id, _)| WeaponId::Mundane(*base_id))
            .chain(
                self.artifact
                    .iter()
                    .map(|(artifact_id, _)| WeaponId::Artifact(*artifact_id)),
            )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons::exalt) struct ExaltUnequippedWeaponsMemo {
    mundane: HashMap<BaseWeaponId, NonnaturalMundaneWeaponMemo>,
    artifact: HashMap<ArtifactWeaponId, NonnaturalArtifactWeaponMemo>,
}

impl<'source> ExaltUnequippedWeaponsMemo {
    pub fn as_ref(&'source self) -> ExaltUnequippedWeapons<'source> {
        ExaltUnequippedWeapons {
            mundane: self.mundane.iter().map(|(k, v)| (*k, v.as_ref())).collect(),
            artifact: self
                .artifact
                .iter()
                .map(|(k, v)| (*k, v.as_ref()))
                .collect(),
        }
    }
}
