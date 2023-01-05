use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::weapons::{
    artifact::{
        ArtifactWeapon, NonnaturalArtifactWeaponNoAttunement,
        NonnaturalArtifactWeaponNoAttunementMemo,
    },
    exalt::ExaltUnequippedWeapons,
    mundane::{MundaneWeapon, NonnaturalMundaneWeapon, NonnaturalMundaneWeaponMemo},
    ArtifactWeaponId, BaseWeaponId, Weapon, WeaponId, WeaponType,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MortalUnequippedWeapons<'source> {
    pub mundane: HashMap<BaseWeaponId, NonnaturalMundaneWeapon<'source>>,
    pub artifact: HashMap<ArtifactWeaponId, NonnaturalArtifactWeaponNoAttunement<'source>>,
}

impl<'source> From<ExaltUnequippedWeapons<'source>> for MortalUnequippedWeapons<'source> {
    fn from(exalt: ExaltUnequippedWeapons<'source>) -> Self {
        Self {
            mundane: exalt.mundane,
            artifact: exalt.artifact.into_iter().map(|(k, v)| (k, v.0)).collect(),
        }
    }
}

impl<'view, 'source> MortalUnequippedWeapons<'source> {
    pub fn as_memo(&self) -> MortalUnequippedWeaponsMemo {
        MortalUnequippedWeaponsMemo {
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

    pub fn get_weapon(&'view self, weapon_id: WeaponId) -> Option<Weapon<'source>> {
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
            WeaponId::Artifact(target_id) => match self.artifact.get(&target_id)? {
                NonnaturalArtifactWeaponNoAttunement::Worn(worn) => Some(Weapon(
                    WeaponType::Artifact(target_id, ArtifactWeapon::Worn(worn.clone(), false), None),
                )),
                NonnaturalArtifactWeaponNoAttunement::OneHanded(one) => Some(Weapon(
                    WeaponType::Artifact(target_id, ArtifactWeapon::OneHanded(one.clone(), None), None),
                )),
                NonnaturalArtifactWeaponNoAttunement::TwoHanded(two) => Some(Weapon(
                    WeaponType::Artifact(target_id, ArtifactWeapon::TwoHanded(two.clone(), false), None),
                )),
            },
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
pub(crate) struct MortalUnequippedWeaponsMemo {
    mundane: HashMap<BaseWeaponId, NonnaturalMundaneWeaponMemo>,
    artifact: HashMap<ArtifactWeaponId, NonnaturalArtifactWeaponNoAttunementMemo>,
}

impl<'source> MortalUnequippedWeaponsMemo {
    pub fn as_ref(&'source self) -> MortalUnequippedWeapons<'source> {
        MortalUnequippedWeapons {
            mundane: self.mundane.iter().map(|(k, v)| (*k, v.as_ref())).collect(),
            artifact: self
                .artifact
                .iter()
                .map(|(k, v)| (*k, v.as_ref()))
                .collect(),
        }
    }
}
