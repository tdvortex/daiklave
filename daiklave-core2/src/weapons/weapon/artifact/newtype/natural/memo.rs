use serde::{Deserialize, Serialize};

use crate::weapons::weapon::artifact::inner::ArtifactWeaponInnerMemo;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NaturalArtifactWeaponMemo(pub(crate) ArtifactWeaponInnerMemo);