use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::newtype::{
    NaturalArtifactWeaponMemo, WornArtifactWeaponMemo,
};

use super::HandlessArtifactWeaponNoAttunement;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum HandlessArtifactWeaponNoAttunementMemo {
    Natural(NaturalArtifactWeaponMemo),
    Worn(WornArtifactWeaponMemo),
}

impl<'source> HandlessArtifactWeaponNoAttunementMemo {
    pub fn as_ref(&'source self) -> HandlessArtifactWeaponNoAttunement<'source> {
        match self {
            HandlessArtifactWeaponNoAttunementMemo::Natural(memo) => {
                HandlessArtifactWeaponNoAttunement::Natural(memo.as_ref())
            }
            HandlessArtifactWeaponNoAttunementMemo::Worn(memo) => {
                HandlessArtifactWeaponNoAttunement::Worn(memo.as_ref())
            }
        }
    }
}
