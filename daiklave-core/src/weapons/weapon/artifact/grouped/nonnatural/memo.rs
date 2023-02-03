use serde::{Deserialize, Serialize};

use super::no_attunement::NonnaturalArtifactWeaponNoAttunementMemo;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NonnaturalArtifactWeaponMemo(
    pub NonnaturalArtifactWeaponNoAttunementMemo,
    pub Option<u8>,
);
