mod equip_hand;
mod memo;
mod no_attunement;

use std::num::NonZeroU8;

pub use equip_hand::EquipHand;
pub use memo::EquippedOneHandedWeaponMemo;
pub(crate) use no_attunement::EquippedOneHandedWeaponNoAttunement;
pub use no_attunement::EquippedOneHandedWeaponNoAttunementMemo;

use crate::weapons::weapon::{
    artifact::{ArtifactWeaponView, OneHandedArtifactWeaponView},
    mundane::{MundaneWeaponView, OneHandedMundaneWeaponView},
    weapon_type::WeaponType,
    ArtifactWeaponId, Weapon, WeaponId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EquippedOneHandedWeapon<'source> {
    Mundane(&'source str, OneHandedMundaneWeaponView<'source>),
    Artifact(
        ArtifactWeaponId,
        OneHandedArtifactWeaponView<'source>,
        Option<u8>,
    ),
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

impl<'view, 'source> EquippedOneHandedWeapon<'source> {
    pub fn as_memo(&self) -> EquippedOneHandedWeaponMemo {
        match self {
            EquippedOneHandedWeapon::Mundane(name, view) => {
                EquippedOneHandedWeaponMemo::Mundane((*name).to_owned(), view.as_memo())
            }
            EquippedOneHandedWeapon::Artifact(id, view, attunement) => {
                EquippedOneHandedWeaponMemo::Artifact(*id, view.as_memo(), *attunement)
            }
        }
    }

    pub fn get_weapon(
        &'view self,
        weapon_id: WeaponId,
        hand: EquipHand,
    ) -> Option<Weapon<'source>> {
        match (self, weapon_id) {
            (EquippedOneHandedWeapon::Mundane(name, one), WeaponId::Mundane(target_name)) => {
                if &target_name != name {
                    None
                } else {
                    Some(Weapon(WeaponType::Mundane(
                        *name,
                        MundaneWeaponView::OneHanded(one.clone(), Some(hand)),
                        NonZeroU8::new(1).unwrap(),
                    )))
                }
            }
            (
                EquippedOneHandedWeapon::Artifact(actual_id, one, attunement),
                WeaponId::Artifact(target_id),
            ) => {
                if &target_id != actual_id {
                    None
                } else {
                    Some(Weapon(WeaponType::Artifact(
                        target_id,
                        ArtifactWeaponView::OneHanded(one.clone(), Some(hand)),
                        *attunement,
                    )))
                }
            }
            (_, _) => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponId<'source>> {
        match self {
            EquippedOneHandedWeapon::Mundane(base_id, _) => {
                std::iter::once(WeaponId::Mundane(*base_id))
            }
            EquippedOneHandedWeapon::Artifact(artifact_id, _, _) => {
                std::iter::once(WeaponId::Artifact(*artifact_id))
            }
        }
    }
}
