use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::inner::ArtifactWeaponInnerMemo;

use super::OneHandedArtifactWeaponView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct OneHandedArtifactWeaponMemo(pub(crate) ArtifactWeaponInnerMemo);

impl From<&OneHandedArtifactWeaponView<'_>> for OneHandedArtifactWeaponMemo {
    fn from(one: &OneHandedArtifactWeaponView<'_>) -> Self {
        Self((&one.0).into())
    }
}