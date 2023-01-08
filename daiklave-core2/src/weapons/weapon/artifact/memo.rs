use crate::weapons::weapon::equipped::EquipHand;

use super::{
    newtype::{
        NaturalArtifactWeapon, OneHandedArtifactWeapon, TwoHandedArtifactWeapon, WornArtifactWeapon,
    },
    ArtifactWeaponView,
};

/// A magical weapon, like a daiklave or a direlash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactWeapon(pub(crate) ArtifactWeaponHandedness);


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactWeaponHandedness {
    Natural(NaturalArtifactWeapon),
    Worn(WornArtifactWeapon, bool),
    OneHanded(OneHandedArtifactWeapon, Option<EquipHand>),
    TwoHanded(TwoHandedArtifactWeapon, bool),
}

impl<'source> ArtifactWeaponHandedness {
    pub(crate) fn as_ref(&'source self) -> ArtifactWeaponView<'source> {
        match self {
            ArtifactWeaponHandedness::Natural(memo) => ArtifactWeaponView::Natural(memo.as_ref()),
            ArtifactWeaponHandedness::Worn(memo, equipped) => {
                ArtifactWeaponView::Worn(memo.as_ref(), *equipped)
            }
            ArtifactWeaponHandedness::OneHanded(memo, equipped) => {
                ArtifactWeaponView::OneHanded(memo.as_ref(), *equipped)
            }
            ArtifactWeaponHandedness::TwoHanded(memo, equipped) => {
                ArtifactWeaponView::TwoHanded(memo.as_ref(), *equipped)
            }
        }
    }
}
