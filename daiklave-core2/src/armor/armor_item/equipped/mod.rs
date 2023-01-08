mod no_attunement;

pub(crate) use no_attunement::EquippedArmorNoAttunement;

use super::{BaseArmorId, mundane::MundaneArmor, artifact::{ArtifactArmorId, ArtifactArmor}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EquippedArmor<'source> {
    Mundane(BaseArmorId, MundaneArmor<'source>),
    Artifact(ArtifactArmorId, ArtifactArmor<'source>),
}