mod equip_hand;
mod memo;
mod no_attunement;

use std::num::NonZeroU8;

pub use equip_hand::EquipHand;
pub(crate) use memo::EquippedOneHandedWeaponMemo;
pub(crate) use no_attunement::EquippedOneHandedWeaponNoAttunement;
pub(crate) use no_attunement::EquippedOneHandedWeaponNoAttunementMemo;

use crate::weapons::weapon::{
    artifact::{ArtifactWeapon, OneHandedArtifactWeaponView},
    mundane::{MundaneWeaponView, OneHandedMundaneWeaponView},
    weapon_type::WeaponType,
    Weapon, WeaponName,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EquippedOneHandedWeapon<'source> {
    Mundane(&'source str, OneHandedMundaneWeaponView<'source>),
    Artifact(
        &'source str,
        OneHandedArtifactWeaponView<'source>,
        Option<u8>,
    ),
}

impl<'source> From<&'source EquippedOneHandedWeaponMemo> for EquippedOneHandedWeapon<'source> {
    fn from(value: &'source EquippedOneHandedWeaponMemo) -> Self {
        match value {
            EquippedOneHandedWeaponMemo::Mundane(name, weapon) => {
                Self::Mundane(name.as_str(), weapon.into())
            }
            EquippedOneHandedWeaponMemo::Artifact(name, weapon, attunement) => {
                Self::Artifact(name.as_str(), weapon.into(), *attunement)
            }
        }
    }
}

impl<'source> From<EquippedOneHandedWeaponNoAttunement<'source>>
    for EquippedOneHandedWeapon<'source>
{
    fn from(unattuned: EquippedOneHandedWeaponNoAttunement<'source>) -> Self {
        match unattuned {
            EquippedOneHandedWeaponNoAttunement::Mundane(id, mundane) => Self::Mundane(id, mundane),
            EquippedOneHandedWeaponNoAttunement::Artifact(id, artifact) => {
                Self::Artifact(id, artifact, None)
            }
        }
    }
}

impl<'source> EquippedOneHandedWeapon<'source> {
    pub fn get_weapon(&self, name: WeaponName<'_>, hand: EquipHand) -> Option<Weapon<'source>> {
        match (self, name) {
            (EquippedOneHandedWeapon::Mundane(name, one), WeaponName::Mundane(target_name)) => {
                if &target_name != name {
                    None
                } else {
                    Some(Weapon(WeaponType::Mundane(
                        name,
                        MundaneWeaponView::OneHanded(one.clone(), Some(hand)),
                        NonZeroU8::new(1).unwrap(),
                    )))
                }
            }
            (
                EquippedOneHandedWeapon::Artifact(name, one, attunement),
                WeaponName::Artifact(target_name),
            ) => {
                if &target_name != name {
                    None
                } else {
                    Some(Weapon(WeaponType::Artifact(
                        name,
                        ArtifactWeapon::OneHanded(one.clone(), Some(hand)),
                        *attunement,
                    )))
                }
            }
            (_, _) => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponName<'source>> {
        match self {
            EquippedOneHandedWeapon::Mundane(name, _) => std::iter::once(WeaponName::Mundane(name)),
            EquippedOneHandedWeapon::Artifact(name, _, _) => {
                std::iter::once(WeaponName::Artifact(name))
            }
        }
    }
}
