mod memo;
use std::collections::{hash_map::Entry, HashMap};

pub(crate) use memo::ExaltUnequippedWeaponsMemo;

use crate::{
    exaltation::mortal::MortalUnequippedWeapons,
    weapons::{
        weapon::{
            artifact::{
                ArtifactWeapon, NonnaturalArtifactWeapon, NonnaturalArtifactWeaponNoAttunement,
            },
            mundane::{MundaneWeapon, NonnaturalMundaneWeapon},
            ArtifactWeaponId, BaseWeaponId, Equipped, Weapon, WeaponId, WeaponType,
        },
        WeaponError,
    },
    CharacterMutationError,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ExaltUnequippedWeapons<'source> {
    pub mundane: HashMap<BaseWeaponId, (NonnaturalMundaneWeapon<'source>, u8)>,
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
                (NonnaturalMundaneWeapon::Worn(worn_weapon), count) => {
                    Some(Weapon(WeaponType::Mundane(
                        target_id,
                        MundaneWeapon::Worn(worn_weapon.clone(), false),
                        *count,
                    )))
                }
                (NonnaturalMundaneWeapon::OneHanded(one), count) => {
                    Some(Weapon(WeaponType::Mundane(
                        target_id,
                        MundaneWeapon::OneHanded(one.clone(), None),
                        *count,
                    )))
                }
                (NonnaturalMundaneWeapon::TwoHanded(two), count) => {
                    Some(Weapon(WeaponType::Mundane(
                        target_id,
                        MundaneWeapon::TwoHanded(two.clone(), false),
                        *count,
                    )))
                }
            },
            WeaponId::Artifact(target_id) => {
                let nonnatural_artifact_weapon = self.artifact.get(&target_id)?;
                let (without_attunement, attunement) =
                    (&nonnatural_artifact_weapon.0, nonnatural_artifact_weapon.1);

                match without_attunement {
                    NonnaturalArtifactWeaponNoAttunement::Worn(worn) => {
                        Some(Weapon(WeaponType::Artifact(
                            target_id,
                            ArtifactWeapon::Worn(worn.clone(), false),
                            attunement,
                        )))
                    }
                    NonnaturalArtifactWeaponNoAttunement::OneHanded(one) => {
                        Some(Weapon(WeaponType::Artifact(
                            target_id,
                            ArtifactWeapon::OneHanded(one.clone(), None),
                            attunement,
                        )))
                    }
                    NonnaturalArtifactWeaponNoAttunement::TwoHanded(two) => {
                        Some(Weapon(WeaponType::Artifact(
                            target_id,
                            ArtifactWeapon::TwoHanded(two.clone(), false),
                            attunement,
                        )))
                    }
                }
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (WeaponId, Option<Equipped>)> + '_ {
        self.mundane
            .iter()
            .map(|(base_id, _)| (WeaponId::Mundane(*base_id), None))
            .chain(
                self.artifact
                    .iter()
                    .map(|(artifact_id, _)| (WeaponId::Artifact(*artifact_id), None)),
            )
    }

    pub fn stow_mundane(
        &mut self,
        weapon_id: BaseWeaponId,
        weapon: NonnaturalMundaneWeapon<'source>,
    ) {
        let count_ptr = &mut self.mundane.entry(weapon_id).or_insert((weapon, 0)).1;
        if *count_ptr < u8::MAX {
            *count_ptr += 1;
        }
    }

    pub fn unstow_mundane(
        &mut self,
        weapon_id: BaseWeaponId,
    ) -> Option<NonnaturalMundaneWeapon<'source>> {
        let current_count = self.mundane.get(&weapon_id)?.1;

        match current_count {
            0 => {
                // This shouldn't happen, but if it does handle it by
                // removing the problem entry
                self.mundane.remove(&weapon_id);
                None
            }
            1 => self.mundane.remove(&weapon_id).map(|(weapon, _)| weapon),
            _ => self.mundane.get_mut(&weapon_id).map(|(weapon, count)| {
                *count -= 1;
                weapon.clone()
            }),
        }
    }

    pub fn stow_artifact(
        &mut self,
        weapon_id: ArtifactWeaponId,
        weapon: NonnaturalArtifactWeapon<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let Entry::Vacant(e) = self.artifact.entry(weapon_id) {
            e.insert(weapon);
            Ok(self)
        } else {
            Err(CharacterMutationError::WeaponError(
                WeaponError::NamedArtifactsUnique,
            ))
        }
    }

    pub fn unstow_artifact(
        &mut self,
        weapon_id: ArtifactWeaponId,
    ) -> Option<NonnaturalArtifactWeapon<'source>> {
        self.artifact.remove(&weapon_id)
    }
}
