use std::num::NonZeroU8;

use crate::weapons::weapon::{
    artifact::{ArtifactWeaponView, OneHandedArtifactWeaponView},
    mundane::{MundaneWeaponView, OneHandedMundaneWeaponView},
    ArtifactWeaponId, Weapon, WeaponId, WeaponType,
};

pub use memo::EquippedOneHandedWeaponNoAttunementMemo;

use super::EquipHand;

mod memo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EquippedOneHandedWeaponNoAttunement<'source> {
    Mundane(&'source str, OneHandedMundaneWeaponView<'source>),
    Artifact(ArtifactWeaponId, OneHandedArtifactWeaponView<'source>),
}

impl<'view, 'source> EquippedOneHandedWeaponNoAttunement<'source> {
    pub fn as_memo(&'source self) -> EquippedOneHandedWeaponNoAttunementMemo {
        match self {
            EquippedOneHandedWeaponNoAttunement::Mundane(name, view) => {
                EquippedOneHandedWeaponNoAttunementMemo::Mundane((*name).to_owned(), view.as_memo())
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
                WeaponId::Mundane(name),
                EquippedOneHandedWeaponNoAttunement::Mundane(actual_name, one),
            ) => {
                if &name != actual_name {
                    None
                } else {
                    Some(Weapon(WeaponType::Mundane(
                        *actual_name,
                        MundaneWeaponView::OneHanded(one.clone(), Some(hand)),
                        NonZeroU8::new(1).unwrap(),
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
                        ArtifactWeaponView::OneHanded(one.clone(), Some(hand)),
                        None,
                    )))
                }
            }
            (_, _) => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponId<'source>> {
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
