use std::num::NonZeroU8;

use crate::weapons::weapon::{
    artifact::{ArtifactWeaponView, TwoHandedArtifactWeaponView},
    mundane::{MundaneWeaponView, TwoHandedMundaneWeaponView},
    weapon_type::WeaponType,
    Weapon, WeaponName,
};

mod memo;
pub use memo::EquippedTwoHandedWeaponNoAttunementMemo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EquippedTwoHandedWeaponNoAttunement<'source> {
    Mundane(&'source str, TwoHandedMundaneWeaponView<'source>),
    Artifact(&'source str, TwoHandedArtifactWeaponView<'source>),
}

impl<'view, 'source> EquippedTwoHandedWeaponNoAttunement<'source> {
    pub fn as_memo(&self) -> EquippedTwoHandedWeaponNoAttunementMemo {
        match self {
            EquippedTwoHandedWeaponNoAttunement::Mundane(name, view) => {
                EquippedTwoHandedWeaponNoAttunementMemo::Mundane((*name).to_owned(), view.as_memo())
            }
            EquippedTwoHandedWeaponNoAttunement::Artifact(name, view) => {
                EquippedTwoHandedWeaponNoAttunementMemo::Artifact(
                    (*name).to_owned(),
                    view.as_memo(),
                )
            }
        }
    }

    pub fn get_weapon(&'view self, name: WeaponName<'_>) -> Option<Weapon<'source>> {
        match (name, self) {
            (WeaponName::Unarmed, _) => Some(crate::weapons::unarmed()),
            (
                WeaponName::Mundane(name),
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
                WeaponName::Artifact(name),
                EquippedTwoHandedWeaponNoAttunement::Artifact(actual_name, two),
            ) => {
                if &name != actual_name {
                    None
                } else {
                    Some(Weapon(WeaponType::Artifact(
                        *actual_name,
                        ArtifactWeaponView::TwoHanded(two.clone(), true),
                        None,
                    )))
                }
            }
            (_, _) => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponName<'source>> {
        match self {
            EquippedTwoHandedWeaponNoAttunement::Mundane(name, _) => {
                std::iter::once(WeaponName::Mundane(*name))
            }
            EquippedTwoHandedWeaponNoAttunement::Artifact(name, _) => {
                std::iter::once(WeaponName::Artifact(*name))
            }
        }
    }
}
