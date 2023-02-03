use serde::{Deserialize, Serialize};

use super::no_attunement::HandlessArtifactWeaponNoAttunementMemo;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct HandlessArtifactWeaponMemo(
    pub HandlessArtifactWeaponNoAttunementMemo,
    pub Option<u8>,
);
