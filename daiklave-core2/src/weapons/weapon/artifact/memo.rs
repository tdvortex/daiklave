use crate::weapons::weapon::equipped::EquipHand;

use super::{
    newtype::{
        NaturalArtifactWeapon, OneHandedArtifactWeapon, TwoHandedArtifactWeapon, WornArtifactWeapon,
    },
    ArtifactWeaponView,
};

/// A magical weapon, like a daiklave or a direlash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactWeapon {
    Natural(NaturalArtifactWeapon),
    Worn(WornArtifactWeapon, bool),
    OneHanded(OneHandedArtifactWeapon, Option<EquipHand>),
    TwoHanded(TwoHandedArtifactWeapon, bool),
}

impl<'source> ArtifactWeapon {
    pub(crate) fn as_ref(&'source self) -> ArtifactWeaponView<'source> {
        match self {
            ArtifactWeapon::Natural(memo) => ArtifactWeaponView::Natural(memo.as_ref()),
            ArtifactWeapon::Worn(memo, equipped) => {
                ArtifactWeaponView::Worn(memo.as_ref(), *equipped)
            }
            ArtifactWeapon::OneHanded(memo, equipped) => {
                ArtifactWeaponView::OneHanded(memo.as_ref(), *equipped)
            }
            ArtifactWeapon::TwoHanded(memo, equipped) => {
                ArtifactWeaponView::TwoHanded(memo.as_ref(), *equipped)
            }
        }
    }
}
