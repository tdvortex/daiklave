use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::inner::ArtifactWeaponInnerMemo;

use super::WornArtifactWeaponView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct WornArtifactWeaponMemo(pub(crate) ArtifactWeaponInnerMemo);

impl From<&WornArtifactWeaponView<'_>> for WornArtifactWeaponMemo {
    fn from(value: &WornArtifactWeaponView<'_>) -> Self {
        Self((&value.0).into())
    }
}