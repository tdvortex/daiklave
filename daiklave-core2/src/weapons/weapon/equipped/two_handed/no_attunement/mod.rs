use std::num::NonZeroU8;

use crate::weapons::weapon::{
    artifact::{ArtifactWeaponView, TwoHandedArtifactWeaponView},
    mundane::{MundaneWeaponView, TwoHandedMundaneWeaponView},
    weapon_type::WeaponType,
    ArtifactWeaponId, Weapon, WeaponId,
};

mod memo;
pub use memo::EquippedTwoHandedWeaponNoAttunementMemo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EquippedTwoHandedWeaponNoAttunement<'source> {
    Mundane(&'source str, TwoHandedMundaneWeaponView<'source>),
    Artifact(ArtifactWeaponId, TwoHandedArtifactWeaponView<'source>),
}

impl<'view, 'source> EquippedTwoHandedWeaponNoAttunement<'source> {
    pub fn as_memo(&self) -> EquippedTwoHandedWeaponNoAttunementMemo {
        match self {
            EquippedTwoHandedWeaponNoAttunement::Mundane(name, view) => {
                EquippedTwoHandedWeaponNoAttunementMemo::Mundane((*name).to_owned(), view.as_memo())
            }
            EquippedTwoHandedWeaponNoAttunement::Artifact(id, view) => {
                EquippedTwoHandedWeaponNoAttunementMemo::Artifact(*id, view.as_memo())
            }
        }
    }

    pub fn get_weapon(&'view self, weapon_id: WeaponId) -> Option<Weapon<'source>> {
        match (weapon_id, self) {
            (WeaponId::Unarmed, _) => Some(crate::weapons::unarmed()),
            (
                WeaponId::Mundane(name),
                EquippedTwoHandedWeaponNoAttunement::Mundane(actual_name, two),
            ) => {
                if &name != actual_name {
                    None
                } else {
                    Some(Weapon(WeaponType::Mundane(
                        *actual_name,
                        MundaneWeaponView::TwoHanded(two.clone(), true),
                        NonZeroU8::new(1).unwrap(),
                    )))
                }
            }
            (
                WeaponId::Artifact(target_id),
                EquippedTwoHandedWeaponNoAttunement::Artifact(actual_id, two),
            ) => {
                if &target_id != actual_id {
                    None
                } else {
                    Some(Weapon(WeaponType::Artifact(
                        target_id,
                        ArtifactWeaponView::TwoHanded(two.clone(), true),
                        None,
                    )))
                }
            }
            (_, _) => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponId<'source>> {
        match self {
            EquippedTwoHandedWeaponNoAttunement::Mundane(base_id, _) => {
                std::iter::once(WeaponId::Mundane(*base_id))
            }
            EquippedTwoHandedWeaponNoAttunement::Artifact(artifact_id, _) => {
                std::iter::once(WeaponId::Artifact(*artifact_id))
            }
        }
    }
}
