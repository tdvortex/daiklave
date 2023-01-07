mod memo;
use std::collections::HashMap;

pub(crate) use memo::MortalUnequippedWeaponsMemo;

use crate::{
    exaltation::exalt::ExaltUnequippedWeapons,
    weapons::weapon::{
        artifact::{ArtifactWeapon, NonnaturalArtifactWeaponNoAttunement},
        mundane::{MundaneWeapon, NonnaturalMundaneWeapon},
        ArtifactWeaponId, BaseWeaponId, Weapon, WeaponId, WeaponType,
    },
    CharacterMutationError,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MortalUnequippedWeapons<'source> {
    pub mundane: HashMap<BaseWeaponId, (NonnaturalMundaneWeapon<'source>, u8)>,
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
                .map(|(k, (v, count))| (*k, (v.as_memo(), *count)))
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
            WeaponId::Unarmed => Some(crate::weapons::weapon::mundane::unarmed()),
            WeaponId::Mundane(target_id) => match self.mundane.get(&target_id)? {
                (NonnaturalMundaneWeapon::Worn(worn_weapon), _) => Some(Weapon(WeaponType::Mundane(
                    target_id,
                    MundaneWeapon::Worn(worn_weapon.clone(), false),
                ))),
                (NonnaturalMundaneWeapon::OneHanded(one), _) => Some(Weapon(WeaponType::Mundane(
                    target_id,
                    MundaneWeapon::OneHanded(one.clone(), None),
                ))),
                (NonnaturalMundaneWeapon::TwoHanded(two), _) => Some(Weapon(WeaponType::Mundane(
                    target_id,
                    MundaneWeapon::TwoHanded(two.clone(), false),
                ))),
            },
            WeaponId::Artifact(target_id) => match self.artifact.get(&target_id)? {
                NonnaturalArtifactWeaponNoAttunement::Worn(worn) => {
                    Some(Weapon(WeaponType::Artifact(
                        target_id,
                        ArtifactWeapon::Worn(worn.clone(), false),
                        None,
                    )))
                }
                NonnaturalArtifactWeaponNoAttunement::OneHanded(one) => {
                    Some(Weapon(WeaponType::Artifact(
                        target_id,
                        ArtifactWeapon::OneHanded(one.clone(), None),
                        None,
                    )))
                }
                NonnaturalArtifactWeaponNoAttunement::TwoHanded(two) => {
                    Some(Weapon(WeaponType::Artifact(
                        target_id,
                        ArtifactWeapon::TwoHanded(two.clone(), false),
                        None,
                    )))
                }
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

    pub fn add_mundane_weapon(
        &mut self,
        weapon_id: BaseWeaponId,
        weapon: NonnaturalMundaneWeapon<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.mundane.entry(weapon_id).or_insert((weapon, 0)).1 += 1;
        Ok(self)
    }
}
