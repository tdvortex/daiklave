use serde::{Deserialize, Serialize};

use super::{no_attunement::ArtifactArmorNoAttunementMemo, ArtifactArmor};

/// An owned copy of a named piece of artifact armor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactArmorMemo(pub ArtifactArmorNoAttunementMemo, pub Option<u8>);

impl<'source> ArtifactArmorMemo {
    pub(crate) fn as_ref(&'source self) -> ArtifactArmor<'source> {
        ArtifactArmor(self.0.as_ref(), self.1)
    }
}
