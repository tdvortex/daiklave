use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::newtype::{
    OneHandedArtifactWeapon, TwoHandedArtifactWeapon, WornArtifactWeapon,
};

use super::NonnaturalArtifactWeaponNoAttunement;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum NonnaturalArtifactWeaponNoAttunementMemo {
    Worn(WornArtifactWeapon),
    OneHanded(OneHandedArtifactWeapon),
    TwoHanded(TwoHandedArtifactWeapon),
}

impl<'source> NonnaturalArtifactWeaponNoAttunementMemo {
    pub fn as_ref(&'source self) -> NonnaturalArtifactWeaponNoAttunement<'source> {
        match self {
            NonnaturalArtifactWeaponNoAttunementMemo::Worn(memo) => {
                NonnaturalArtifactWeaponNoAttunement::Worn(memo.as_ref())
            }
            NonnaturalArtifactWeaponNoAttunementMemo::OneHanded(memo) => {
                NonnaturalArtifactWeaponNoAttunement::OneHanded(memo.as_ref())
            }
            NonnaturalArtifactWeaponNoAttunementMemo::TwoHanded(memo) => {
                NonnaturalArtifactWeaponNoAttunement::TwoHanded(memo.as_ref())
            }
        }
    }
}
