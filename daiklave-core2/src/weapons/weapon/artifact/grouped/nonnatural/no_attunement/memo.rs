use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::newtype::{
    OneHandedArtifactWeaponMemo, TwoHandedArtifactWeaponMemo, WornArtifactWeaponMemo,
};

use super::NonnaturalArtifactWeaponNoAttunement;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum NonnaturalArtifactWeaponNoAttunementMemo {
    Worn(WornArtifactWeaponMemo),
    OneHanded(OneHandedArtifactWeaponMemo),
    TwoHanded(TwoHandedArtifactWeaponMemo),
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
