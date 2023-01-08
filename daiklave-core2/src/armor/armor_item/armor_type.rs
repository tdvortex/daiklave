use super::{artifact::{ArtifactArmorId, ArtifactArmorNoAttunement}, BaseArmorId, mundane::MundaneArmor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ArmorType<'source> {
    Artifact(ArtifactArmorId, ArtifactArmorNoAttunement<'source>, Option<u8>),
    Mundane(BaseArmorId, MundaneArmor<'source>)
}