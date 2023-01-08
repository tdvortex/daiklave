use serde::{Deserialize, Serialize};

use super::{no_attunement::ArtifactArmorNoAttunementMemo, ArtifactArmorView};

/// A piece of magical armor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactArmor(
    pub(crate) ArtifactArmorNoAttunementMemo,
    pub(crate) Option<u8>,
);

impl<'source> ArtifactArmor {
    pub(crate) fn as_ref(&'source self) -> ArtifactArmorView<'source> {
        ArtifactArmorView(self.0.as_ref(), self.1)
    }
}
