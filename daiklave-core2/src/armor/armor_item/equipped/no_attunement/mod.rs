use crate::armor::armor_item::{mundane::MundaneArmor, artifact::{ArtifactArmorNoAttunement, ArtifactArmorId}, BaseArmorId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EquippedArmorNoAttunement<'source> {
    Mundane(BaseArmorId, MundaneArmor<'source>),
    Artifact(ArtifactArmorId, ArtifactArmorNoAttunement<'source>),
}