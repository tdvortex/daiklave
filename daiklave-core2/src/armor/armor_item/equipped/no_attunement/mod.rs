mod memo;
pub(crate) use memo::EquippedArmorNoAttunementMemo;

use crate::armor::armor_item::{
    artifact::{ArtifactArmorId, ArtifactArmorNoAttunement},
    mundane::MundaneArmorView,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EquippedArmorNoAttunement<'source> {
    Mundane(&'source str, MundaneArmorView<'source>),
    Artifact(ArtifactArmorId, ArtifactArmorNoAttunement<'source>),
}

impl<'source> EquippedArmorNoAttunement<'source> {
    pub fn as_memo(&self) -> EquippedArmorNoAttunementMemo {
        match self {
            EquippedArmorNoAttunement::Mundane(name, mundane_armor) => {
                EquippedArmorNoAttunementMemo::Mundane((*name).to_owned(), mundane_armor.as_memo())
            }
            EquippedArmorNoAttunement::Artifact(artifact_armor_id, artifact_armor) => {
                EquippedArmorNoAttunementMemo::Artifact(
                    *artifact_armor_id,
                    artifact_armor.as_memo(),
                )
            }
        }
    }
}
