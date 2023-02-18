use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::inner::ArtifactWeaponInnerMemo;

use super::TwoHandedArtifactWeaponView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TwoHandedArtifactWeaponMemo(pub(crate) ArtifactWeaponInnerMemo);

impl From<&TwoHandedArtifactWeaponView<'_>> for TwoHandedArtifactWeaponMemo {
    fn from(value: &TwoHandedArtifactWeaponView<'_>) -> Self {
        Self((&value.0).into())
    }
}