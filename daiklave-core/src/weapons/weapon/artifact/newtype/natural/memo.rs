use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::inner::ArtifactWeaponInnerMemo;

use super::NaturalArtifactWeaponView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NaturalArtifactWeaponMemo(pub(crate) ArtifactWeaponInnerMemo);

impl From<&NaturalArtifactWeaponView<'_>> for NaturalArtifactWeaponMemo {
    fn from(value: &NaturalArtifactWeaponView<'_>) -> Self {
        Self((&value.0).into())
    }
}