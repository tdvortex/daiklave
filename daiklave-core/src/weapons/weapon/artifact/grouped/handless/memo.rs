use serde::{Deserialize, Serialize};

use super::{no_attunement::HandlessArtifactWeaponNoAttunementMemo, HandlessArtifactWeapon};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct HandlessArtifactWeaponMemo(
    pub HandlessArtifactWeaponNoAttunementMemo,
    pub Option<u8>,
);

impl From<&HandlessArtifactWeapon<'_>> for HandlessArtifactWeaponMemo {
    fn from(value: &HandlessArtifactWeapon<'_>) -> Self {
        Self((&value.0).into(), value.1)
    }
}