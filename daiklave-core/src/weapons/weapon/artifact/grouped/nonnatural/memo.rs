use serde::{Deserialize, Serialize};

use super::{no_attunement::NonnaturalArtifactWeaponNoAttunementMemo, NonnaturalArtifactWeapon};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NonnaturalArtifactWeaponMemo(
    pub NonnaturalArtifactWeaponNoAttunementMemo,
    pub Option<u8>,
);

impl From<&NonnaturalArtifactWeapon<'_>> for NonnaturalArtifactWeaponMemo {
    fn from(value: &NonnaturalArtifactWeapon<'_>) -> Self {
        Self((&value.0).into(), value.1)
    }
}