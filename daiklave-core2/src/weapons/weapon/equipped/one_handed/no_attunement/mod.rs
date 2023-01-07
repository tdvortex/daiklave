use crate::weapons::weapon::{
    artifact::{ArtifactWeapon, OneHandedArtifactWeapon},
    mundane::{MundaneWeapon, OneHandedMundaneWeapon},
    ArtifactWeaponId, BaseWeaponId, Weapon, WeaponId, WeaponType,
};

pub use memo::EquippedOneHandedWeaponNoAttunementMemo;

use super::EquipHand;

mod memo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EquippedOneHandedWeaponNoAttunement<'source> {
    Mundane(BaseWeaponId, OneHandedMundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, OneHandedArtifactWeapon<'source>),
}

impl<'view, 'source> EquippedOneHandedWeaponNoAttunement<'source> {
    pub fn as_memo(&'source self) -> EquippedOneHandedWeaponNoAttunementMemo {
        match self {
            EquippedOneHandedWeaponNoAttunement::Mundane(id, view) => {
                EquippedOneHandedWeaponNoAttunementMemo::Mundane(*id, view.as_memo())
            }
            EquippedOneHandedWeaponNoAttunement::Artifact(id, view) => {
                EquippedOneHandedWeaponNoAttunementMemo::Artifact(*id, view.as_memo())
            }
        }
    }

    pub fn get_weapon(
        &'view self,
        weapon_id: WeaponId,
        hand: EquipHand,
    ) -> Option<Weapon<'source>> {
        match (weapon_id, self) {
            (WeaponId::Unarmed, _) => Some(crate::weapons::unarmed()),
            (
                WeaponId::Mundane(target_id),
                EquippedOneHandedWeaponNoAttunement::Mundane(actual_id, one),
            ) => {
                if &target_id != actual_id {
                    None
                } else {
                    Some(Weapon(WeaponType::Mundane(
                        target_id,
                        MundaneWeapon::OneHanded(one.clone(), Some(hand)),
                        1
                    )))
                }
            }
            (
                WeaponId::Artifact(target_id),
                EquippedOneHandedWeaponNoAttunement::Artifact(actual_id, one),
            ) => {
                if &target_id != actual_id {
                    None
                } else {
                    Some(Weapon(WeaponType::Artifact(
                        target_id,
                        ArtifactWeapon::OneHanded(one.clone(), Some(hand)),
                        None,
                    )))
                }
            }
            (_, _) => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponId> {
        match self {
            EquippedOneHandedWeaponNoAttunement::Mundane(base_id, _) => {
                std::iter::once(WeaponId::Mundane(*base_id))
            }
            EquippedOneHandedWeaponNoAttunement::Artifact(artifact_id, _) => {
                std::iter::once(WeaponId::Artifact(*artifact_id))
            }
        }
    }
}
