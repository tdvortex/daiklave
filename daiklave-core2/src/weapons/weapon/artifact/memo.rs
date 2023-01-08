use crate::weapons::weapon::equipped::EquipHand;

use super::{
    newtype::{
        NaturalArtifactWeaponMemo, OneHandedArtifactWeaponMemo, TwoHandedArtifactWeaponMemo,
        WornArtifactWeaponMemo,
    },
    ArtifactWeapon,
};

/// An owned copy of an artifact weapon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactWeaponMemo {
    Natural(NaturalArtifactWeaponMemo),
    Worn(WornArtifactWeaponMemo, bool),
    OneHanded(OneHandedArtifactWeaponMemo, Option<EquipHand>),
    TwoHanded(TwoHandedArtifactWeaponMemo, bool),
}

impl<'source> ArtifactWeaponMemo {
    pub(crate) fn as_ref(&'source self) -> ArtifactWeapon<'source> {
        match self {
            ArtifactWeaponMemo::Natural(memo) => ArtifactWeapon::Natural(memo.as_ref()),
            ArtifactWeaponMemo::Worn(memo, equipped) => {
                ArtifactWeapon::Worn(memo.as_ref(), *equipped)
            }
            ArtifactWeaponMemo::OneHanded(memo, equipped) => {
                ArtifactWeapon::OneHanded(memo.as_ref(), *equipped)
            }
            ArtifactWeaponMemo::TwoHanded(memo, equipped) => {
                ArtifactWeapon::TwoHanded(memo.as_ref(), *equipped)
            }
        }
    }
}
