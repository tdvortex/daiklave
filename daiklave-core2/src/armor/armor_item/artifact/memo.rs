use serde::{Deserialize, Serialize};

use super::{no_attunement::ArtifactArmorNoAttunementMemo, ArtifactArmorView};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ArtifactArmor(
    pub(crate) ArtifactArmorNoAttunementMemo,
    pub(crate) Option<u8>,
);

impl From<&ArtifactArmorView<'_>> for ArtifactArmor {
    fn from(view: &ArtifactArmorView<'_>) -> Self {
        Self((&view.0).into(), view.1)
    }
}